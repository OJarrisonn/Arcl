Boxes are the ARCL variables, but why we call them boxes? Just because it's cool.
Boxes hold information, but information can be the information on itself, or information about other information, and that's were *holding* and *pointing* boxes come to the party.
Boxes own some information that can be referenced by other boxes while the box is still valid (a valid box is a box which owns some information).
The information ownership can be moved to other boxes or to function arguments.
# Holding Boxes
The boxes that hold the information on itself, basicly they are the owners of that information. 
# Pointing Boxes
The boxes which don't store the information, but they know who do store. And there are 2 types of *pointing* boxes:
## Writing Boxes
Boxes that point to other boxes that contain information. It doesn't mean that the box being pointed must be a holding box. They can modify the information on the box being pointed.
## Reading Boxes
They can just access the information but can't modify it.

# Lifetime
Once a box is created it still exists until it's redeclared, undeclared or get out of scope. A box can be marked as *movable* so it can be moved through scopes (but if they aren't moved, they are undeclared as usual).