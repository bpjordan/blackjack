use super::cards::{Card, CardFace};

const CARD_ICON_10: &str = "\
10 	   

   h h   
  h h h  
  h h h  
   h h   

       10";

const CARD_ICON_9: &str = "\
9        

   h h   
   h h   
  h h h  
   h h   

        9";

const CARD_ICON_8: &str = "\
8        

   h h   
   h h   
   h h   
   h h   

        8";

const CARD_ICON_7: &str = "\
7        

   h h   
   h h   
    h    
   h h   

        7";

const CARD_ICON_6: &str = "\
6        

   h h   
   h h   
   h h   

         
        6";

const CARD_ICON_5: &str = "\
5        

   h h   
    h    
   h h   
         

        5";

const CARD_ICON_4: &str = "\
4        

   h h   
         
         
   h h   

        4";

const CARD_ICON_3: &str = "\
3        

    h    
    h    
    h    
         

        3";

const CARD_ICON_2: &str = "\
2        

    h    
         
         
    h    

        2";

const CARD_ICON_A: &str = "\
A        

         
    h    
         
         

        A";

const CARD_ICON_K: &str = "\
K        

   h h   
  h h h  
  h h h  
   h h   

        K";

const CARD_ICON_Q: &str = "\
Q        

   h h   
  h h h  
  h h h  
   h h   

        Q";

const CARD_ICON_J: &str = "\
J        

   h h   
  h h h  
  h h h  
   h h   

        J";

const CARD_ICON_FLIPPED_ASCII: &str = "

 /\\_/\\ 
(=^.^=)
(\") (\")_/
         

         ";

const CARD_ICON_FLIPPED: &str = "
 ♥ ♦ ♣ ♠ 
         
 /\\_/\\ 
(=^.^=)  
(\") (\")_/
 ♥ ♦ ♣ ♠ 

";

impl Card {
    pub fn icon(&self, ascii: bool) -> String {
        let template = match self.face() {
            CardFace::Jack => CARD_ICON_J,
            CardFace::King => CARD_ICON_K,
            CardFace::Queen => CARD_ICON_Q,
            CardFace::Ace => CARD_ICON_A,
            CardFace::Number(10) => CARD_ICON_10,
            CardFace::Number(9) => CARD_ICON_9,
            CardFace::Number(8) => CARD_ICON_8,
            CardFace::Number(7) => CARD_ICON_7,
            CardFace::Number(6) => CARD_ICON_6,
            CardFace::Number(5) => CARD_ICON_5,
            CardFace::Number(4) => CARD_ICON_4,
            CardFace::Number(3) => CARD_ICON_3,
            CardFace::Number(2) => CARD_ICON_2,
            CardFace::Number(_) => {
                if ascii {
                    CARD_ICON_FLIPPED_ASCII
                } else {
                    CARD_ICON_FLIPPED
                }
            },
        };

        let suit_str = if ascii {
            format!("{:#}", self.suit())
        } else {
            format!("{}", self.suit())
        };

        template.replace("h", &suit_str)
    }

    pub fn flipped_icon(ascii: bool) -> &'static str {
        if ascii {
            CARD_ICON_FLIPPED_ASCII
        } else {
            CARD_ICON_FLIPPED
        }
    }
}