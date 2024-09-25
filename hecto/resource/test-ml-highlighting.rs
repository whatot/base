/* This is a regular old ML comment

which goes on

until
there 👇
*/
struct foo; /* ml comments do not have to span multiple lines */
struct /*  they can show up in the middle of the line */ bar; 
struct baz; /* or they start in the middle of a line


and end in the middle of a line*/ struct f00; 

/* they can contain things which should be ignored
- keywords like struct
- single line comments: //  
- char definition: '
*/

/* and even worse: There are nested comments:
    /* which start in the middle  and end in the middle of an existing ML comment
    */
    and once they end, the original comment is still there.
*/

/* you need to highlight this correctly: /*/*// /**//**///*/*/*/*/*/*/**/*/*/*/*/*/*/*/*/*/ struct not_part_of_comment; /* part of a comment */