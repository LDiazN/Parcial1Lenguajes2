/*

Grammar:                                                                                LA
    S -> I $                                { S.type <- I.type }                    |   try, instr
    I ->  try I1 catch I2 F Z               {                                       |   try
                                                F.in <- Either I1.type I2.type      |
                                                Z.in <- F.type                      |
                                                I.type <- Z.type                    |
                                            }                                       |   
      |   instr Z                           {                                       |   instr
                                                Z.in   <- instr.type                |
                                                I.type <- Z.type                    |
                                            }                                       |
                                                                                    |
    F -> finally I3                         { F.type <- I3.type }                   |   finally 
      |  lambda                             { F.type <- F.in }                      |   ;, $, catch, 
    Z ->  ; I1                              { Z.type  <- I1.type }                  |   ;
      |  lambda                             { Z.type  <- Z.in }                     |   $, catch, finally   
                                                                                    |

    

*/


type TypeName = String;
type Stack<T> = Vec<T>;



#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Symbol {
    Instr(TypeName),
    Try,
    Catch,
    Finally,
    SemiColon,
    End,
    NonTerminal(char)
}

#[derive(Debug,Clone)]
pub struct ItemData {
    inh : TypeName,         
    type_val : TypeName
}

#[derive(Debug)]
pub struct Parser {
    stack : Stack<Symbol>,                  // stacked symbols 
    curr_lookahead : usize, // input index
    data_stack : Stack<Option<ItemData>>
}

const S : Symbol = Symbol::NonTerminal('S');
const I : Symbol = Symbol::NonTerminal('I');
const Z : Symbol = Symbol::NonTerminal('Z');
const F : Symbol = Symbol::NonTerminal('F');


impl Symbol {
    /// Convert from string to steam of tokens
    pub fn from(input : &str) -> Vec<Symbol>{
        let mut v : Vec<Symbol> = input
            .replace(";", " ; ")
            .split_whitespace()
            .map(|s| 
                match s.to_lowercase().as_str() {
                    "try"       => Symbol::Try,
                    "catch"     => Symbol::Catch,
                    "finally"   => Symbol::Finally,
                    ";"         => Symbol::SemiColon,
                    _           => Symbol::Instr(s.to_string())
            })
            .collect();
            
        v.push(Symbol::End);

        v
    }
} 

impl ItemData {

    // create empty item
    pub fn new() -> ItemData {
        ItemData {
            inh : format!(""),
            type_val : format!("")
        }
    }
}

pub fn to_either (e1 : String, e2 : String) -> TypeName {
    format!("Eiter {} {}", e1, e2)
}

impl Parser {

    // Create a new parser state
    pub fn new () -> Parser {
        Parser {
            stack : vec![ S ],
            curr_lookahead : 0,
            data_stack : vec![ Some(ItemData::new())]
        }
    }

    // parse a string
    pub fn parse_string(&mut self, input : &str) -> Option<TypeName> {
        self.parse(&Symbol::from(input))
    }

    // Parse a vector of symbols
    pub fn parse(&mut self, input : &Vec<Symbol>) -> Option<TypeName>{

        println!("Parser state: \n\t {:?}", self);
        println!("Arg: \n\t {:?}", &input[self.curr_lookahead..]);
        // next input symbol, next stack symbol
        let state = (input[self.curr_lookahead].clone(), self.stack.last().expect("should be something stacked"));
        

        match state {
            (Symbol::Try, Symbol::NonTerminal('S')) | (Symbol::Instr(_), Symbol::NonTerminal('S')) => {

                // add items to stacks
                self.stack.push(Symbol::End); self.data_stack.push(None);
                self.stack.push(I)          ; self.data_stack.push(Some(ItemData::new()));

                // parse I
                let i_type = self.parse(input)?;

                // delete I
                self.stack.pop(); self.data_stack.pop();

                // parse end
                self.parse(input); self.data_stack.pop(); self.stack.pop();

                // update s_data
                match self.data_stack.last_mut().expect("Data stack inconsistent to symbol stack") {
                    None =>     return None,
                    Some(d) => d.type_val = i_type
                }

            },
            (Symbol::Try, Symbol::NonTerminal('I')) => {

                // Push new symbols
                self.stack.push(Z);             self.data_stack.push(Some(ItemData::new()));
                self.stack.push(F);             self.data_stack.push(Some(ItemData::new()));
                self.stack.push(I);             self.data_stack.push(Some(ItemData::new()));
                self.stack.push(Symbol::Catch); self.data_stack.push(None);
                self.stack.push(I);             self.data_stack.push(Some(ItemData::new()));
                self.stack.push(Symbol::Try);   self.data_stack.push(None);

                // Parse
                self.parse(input);                  self.stack.pop(); self.data_stack.pop(); // parse try & delete it as it is not necessary anymore
                let i1_type = self.parse(input)?;   self.stack.pop(); self.data_stack.pop(); // parse i1 & delete 
                self.parse(input);                  self.stack.pop(); self.data_stack.pop(); // parse catch & delete
                let i2_type = self.parse(input)?;   self.stack.pop(); self.data_stack.pop(); // parse i2 & delete

                // update F.in <- Either I1.type I2.type
                let f = self.data_stack.last_mut().expect("should be something in data stack");
                match f {
                    None => return None,
                    Some(f_data) => f_data.inh = to_either(i1_type, i2_type)
                };

                let f_type = self.parse(input)?; self.stack.pop(); self.data_stack.pop(); // parse F & delete

                // update Z.in <- F.type
                let z = self.data_stack.last_mut().expect("should be something in data stack");
                match z {
                    None => return None,
                    Some(z_data) => z_data.inh = f_type
                };
                let z_type = self.parse(input)?; self.stack.pop(); self.data_stack.pop(); // parse Z & delete

                // update i values
                let i = self.data_stack.last_mut().expect("should be something in data stack"); // store new data for i
                match i {
                    None => return None,
                    Some(i_data) => i_data.type_val = z_type
                };

            },

            (Symbol::Instr(s), Symbol::NonTerminal('I')) => {
                // Push new symbols
                self.stack.push(Z);                         self.data_stack.push(Some(ItemData::new()));
                self.stack.push(Symbol::Instr(s.clone()));  self.data_stack.push(Some(ItemData::new()));

                // Parse instr
                self.parse(input);

                // get instr type 
                let instr_type = self.data_stack.last()
                                                .expect("Data stack inconsistent to symbol stack")
                                                .clone()?
                                                .type_val;

                // Pop instr 
                self.stack.pop(); self.data_stack.pop();

                // Update Z.in <- instr.type
                match self.data_stack.last_mut().expect("data stack inconsistent to symbol stack") {
                    None => return None,
                    Some(d) => d.inh = instr_type
                };

                // parse Z
                let z_type = self.parse(input)?; 
                
                // Delete Z
                self.stack.pop(); self.data_stack.pop();

                
                // update symbol I.type <- Z.type
                match self.data_stack.last_mut().expect("data stack inconsistent to symbol stack") {
                    None => return None,
                    Some(d) => d.type_val = z_type
                };

                println!("CURRENT STATE IS: \n\t{:?}", self);
            },
            (Symbol::Finally, Symbol::NonTerminal('F')) => {
                // push symbols
                self.stack.push(I);                 self.data_stack.push(Some(ItemData::new()));
                self.stack.push(Symbol::Finally);   self.data_stack.push(None);

                // Parse finally
                self.parse(input);

                // pop finally
                self.stack.pop(); self.data_stack.pop();

                // parse I
                let i_type = self.parse(input)?;

                // delete I
                self.stack.pop(); self.data_stack.pop();

                // update F.type <- I.type
                match self.data_stack.last_mut().expect("data stack inconsistent to symbol stack") {
                    None => return None,
                    Some(d) => d.type_val = i_type
                };
            },
            (Symbol::End,       Symbol::NonTerminal('F'))     | 
            (Symbol::SemiColon, Symbol::NonTerminal('F'))     |
            (Symbol::Catch,     Symbol::NonTerminal('F'))     |
            (Symbol::End,       Symbol::NonTerminal('Z'))     | 
            (Symbol::Finally,   Symbol::NonTerminal('Z'))     |
            (Symbol::Catch,     Symbol::NonTerminal('Z'))     => {
                // update Z.type <- Z.in
                match self.data_stack.last_mut().expect("data stack inconsistent to symbol stack") {
                    None    => return None,
                    Some(d) => d.type_val = d.inh.clone()
                }
            },
            (Symbol::SemiColon, Symbol::NonTerminal('Z'))    => {
                // push I
                self.stack.push(I); self.data_stack.push(Some(ItemData::new()));
                // Push semicolon
                self.stack.push(Symbol::SemiColon); self.data_stack.push(None);

                // parse semicolon
                self.parse(input); 

                // delete semicolon
                self.stack.pop(); self.data_stack.pop();
                
                // Parse I
                let i_type = self.parse(input)?;
                
                // Delete I
                self.stack.pop(); self.data_stack.pop();
                
                // update Z.type <- I.type
                match self.data_stack.last_mut().expect("data stack inconsistent to symbol stack") {
                    None => return None,
                    Some(d) => d.type_val = i_type
                }

            },
            (Symbol::Try, Symbol::Try)                  | 
            (Symbol::Catch, Symbol::Catch)              | 
            (Symbol::Finally, Symbol::Finally)          | 
            (Symbol::SemiColon, Symbol::SemiColon)      | 
            (Symbol::End, Symbol::End) => {
                self.curr_lookahead += 1;
            },
            (Symbol::Instr(s0), Symbol::Instr(s1)) => {

                // if instr is not the same type as stored one, raise an error
                if s0 != *s1 { return None }
                // Update data staco to get type of this instr
                match self.data_stack.last_mut().expect("Data stack inconsistent to symbol stack") {
                    None =>     return None,
                    Some(d) => d.type_val = s0
                }

                // move to next symbol
                self.curr_lookahead += 1;
            }
            _ => return None
        }

        return Some( self
                        .data_stack
                        .last()
                        .expect("should be something stacked")
                        .clone()?
                        .type_val
                    )
    }

}