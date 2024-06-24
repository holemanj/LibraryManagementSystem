This is based on the capstone project from an online C++ course 
I took years ago.  Unfortunately, I can remember neither the name 
of the organization or the instructor... if you happen to see
this, send me a message and I will update the attribution accordingly.

This is a nice exercise of programming ability, since it touches on 
several different concepts: file handling, string manipulation, data
structures and algorithms. A clear design helps with keeping the program
elements small and understandable, as does a good knowledge of available
libraries.

Each language that I know, or will teach myself will have its own folder,
which will also include any packaging items unique to that language (eq
Cargo configuration for the Rust language).

The initial specification is:
Information about the inventory of a library will be held in a text file.
Each inventory item will occupy a line in the text file, and have the
format:
Title|Author|Publisher|Publication Year|Number of Copies|
(the delimiter is the vertical bar or pipe).

The user will be able to:
1) View the inventory
2) Add an item:
  * If it is already in the inventory, then the count is incremented
  * Otherwise, a new item is added to the inventory
3) Remove an item - if the last item is removed, the count should be
    set to zero rather than removing the item completely
4) Clean up - deletes from the file any items with zero count 
5) Save changes to the file

Design considerations:
- The program should be able to readily be adapted changes to the file structure
  (fields and delimited).
- All output items should be made accessible for any type of user front-end (the
  program should be adaptable to be used as a web backend, or a windowed
  aplication)
