use std::collections::HashMap;

lazy_static! {
    pub static ref LETTERS: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert(' ', "");
        m.insert('a', "
 ###
#   #
#####
#   #
#   #
");
        m.insert('b', "
####
#   #
#####
#   #
####
");
        m.insert('c', "
 ###
#   #
#
#   #
 ###
");
        m.insert('d', "
####
#   #
#   #
#   #
####
");
        m.insert('e', "
#####
#
#####
#
#####
");
        m.insert('f', "
#####
#
#####
#
#
");
        m.insert('g', "
 ###
#
#  ##
#   #
 ###
");
        m.insert('h', "
#   #
#   #
#####
#   #
#   #
");
        m.insert('i', "
 ###
  #
  #
  #
  #
 ###
");
        m.insert('j', "
    #
    #
    #
    #
#   #
 ###
");
        m.insert('k', "
#   #
#  #
###
#  #
#   #
");
        m.insert('l', "
#
#
#
#
#####
");
        m.insert('m', "
#   #
## ##
# # #
#   #
#   #
");
        m.insert('n', "
#   #
##  #
# # #
#  ##
#   #
");
        m.insert('o', "
 ###
#   #
#   #
#   #
 ###
");
        m.insert('p', "
#####
#   #
#####
#
#
");
        m.insert('q', "
 ###
#   #
#   #
#  ##
 ## #
");
        m.insert('r', "
####
#   #
####
#  #
#   #
");
        m.insert('s', "
 ###
#
 ###
    #
####
");
        m.insert('t', "
#####
  #
  #
  #
  #
");
        m.insert('u', "
#   #
#   #
#   #
#   #
 ###
");
        m.insert('v', "
#   #
#   #
#   #
 # #
  #
");
        m.insert('w', "
#   #
#   #
# # #
## ##
#   #
");
        m.insert('x', "
#   #
 # #
  #
 # #
#   #
");
        m.insert('y', "
#   #
 # #
  #
  #
  #
");
        m.insert('z', "
#####
   #
  #
 #
#####
");
        m.insert('A', "
 ###
#   #
#####
#   #
#   #
");
        m.insert('B', "
####
#   #
#####
#   #
####
");
        m.insert('C', "
 ###
#   #
#
#   #
 ###
");
        m.insert('D', "
####
#   #
#   #
#   #
####
");
        m.insert('E', "
#####
#
#####
#
#####
");
        m.insert('F', "
#####
#
#####
#
#
");
        m.insert('G', "
 ###
#
#  ##
#   #
 ###
");
        m.insert('H', "
#   #
#   #
#####
#   #
#   #
");
        m.insert('I', "
 ###
  #
  #
  #
  #
 ###
");
        m.insert('J', "
    #
    #
    #
    #
#   #
 ###
");
        m.insert('K', "
#   #
#  #
###
#  #
#   #
");
        m.insert('L', "
#
#
#
#
#####
");
        m.insert('M', "
#   #
## ##
# # #
#   #
#   #
");
        m.insert('N', "
#   #
##  #
# # #
#  ##
#   #
");
        m.insert('O', "
 ###
#   #
#   #
#   #
 ###
");
        m.insert('P', "
#####
#   #
#####
#
#
");
        m.insert('Q', "
 ###
#   #
#   #
#  ##
 ## #
");
        m.insert('R', "
####
#   #
####
#  #
#   #
");
        m.insert('S', "
 ###
#
 ###
    #
####
");
        m.insert('T', "
#####
  #
  #
  #
  #
");
        m.insert('U', "
#   #
#   #
#   #
#   #
 ###
");
        m.insert('V', "
#   #
#   #
#   #
 # #
  #
");
        m.insert('W', "
#   #
#   #
# # #
## ##
#   #
");
        m.insert('X', "
#   #
 # #
  #
 # #
#   #
");
        m.insert('Y', "
#   #
 # #
  #
  #
  #
");
        m.insert('Z', "
#####
   #
  #
 #
#####
");
        m.insert('1', "
     #
     #
     #
     #
     #
");
        m.insert('2', "
######
     #
######
#
######
");
        m.insert('3', "
######
     #
######
     #
######
");
        m.insert('4', "
#    #
#    #
######
     #
     #
");
        m.insert('5', "
######
#
######
     #
######
");
        m.insert('6', "
######
#
######
#    #
######
");
        m.insert('7', "
######
     #
     #
     #
     #
");
        m.insert('8', "
######
#    #
######
#    #
######
");
        m.insert('9', "
######
#    #
######
     #
     #
");
        m.insert('0', "
######
#    #
#    #
#    #
######
");
        m.insert(':', "

  ##

  ##

");
       m.insert('~', "
 ##
#  #  #
    ##
");
        m.insert('!', "
###
###
 #

 #
");
        m.insert('@', "
 #####
#     #
# ### #
# ### #
# ####
#
 #####
");
        m.insert('#', "
 # #
#####
 # #
#####
 # #
");
        m.insert('$', "
 #####
#  #  #
#  #
 #####
   #  #
#  #  #
 #####
");
        m.insert('%', "
#   #
   #
  #
 #
#   #
");
        m.insert('^', "
  #
 # #
#   #


");
        m.insert('&', "
  ##
 #  #
 ###
#   #
 ### #
");
        m.insert('*', "
 # #
#####
 # #
");
        m.insert(')', "
##
  #
  #
  #
##
");
        m.insert('(', "
  ##
 #
 #
 #
  ##
");
        m.insert('+', "
  #
  #
#####
  #
  #
");
        m.insert('{', "
  ###
 #
##
 #
  ###
");
        m.insert('}', "
###
   #
   ##
   #
###
");
        m.insert('[', "
###
#
#
#
###
");
        m.insert(']', "
###
  #
  #
  #
###
");
        m.insert(';', "
##
##

##
#

");
        m.insert('\'', "
##
 #
#
");
        m.insert('"', "
## ##
## ##
 #  #
");
        m.insert('.', "




 #
");
        m.insert('<', "
  #
 #
#
 #
  #
");
        m.insert('>', "
 #
  #
   #
  #
 #
");
        m.insert('/', "
    #
   #
  #
 #
#
");
        m.insert('?', "
 ####
#    #
   #

   #
");
        m.insert('\\', "
#
 #
  #
   #
    #
");
        m
    };

    /// Convert above letters into positions to move the cursor
    pub static ref LETTERS_POS: HashMap<char, Vec<(i32, i32)>> = {

        let mut m: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for (key, block) in LETTERS.iter() {

            let mut positions: Vec<(i32, i32)> = vec![];
            let mut x: i32 = 0;
            let mut y: i32 = 0;

            for c in block.to_string().chars().skip(1) {
                match c {
                    ' ' => {
                        x += 1;
                    }
                    '\n' => {
                        x = 0;
                        y += 1;
                    }
                    _ => {
                        positions.push((x, y));
                        x += 1;
                    }
                }
            }

            m.insert(*key, positions);
        }
        m
    };
}
