with Ada.Text_IO; use Ada.Text_IO;
with Input_Sources.File;  use Input_Sources.File;
with Sax.Readers;        use Sax.Readers;
with DOM.Readers;         use DOM.Readers;
with DOM.Core;            use DOM.Core;
with DOM.Core.Documents; use DOM.Core.Documents;
with DOM.Core.Nodes;     use DOM.Core.Nodes;
with DOM.Core.Attrs;     use DOM.Core.Attrs;

procedure ReadPrefs is
   Filename : String := "setup.xml";
   Input : File_Input;
   Reader : Tree_Reader;
   Doc : Document;
   Structure : Node;
   Structure_Items : Node_List;
   List   : Node_List;
   N      : Node;
   A      : Attr;
begin
   Put_Line ("XML stuffs");

   Open (Filename, Input);
   Parse (Reader, Input);
   Close (Input);

   Doc := Get_Tree (Reader);
   Structure_Items := Get_Elements_By_Tag_Name (Doc, "structure");

   for Index in 1 .. Length (Structure_Items) loop
      N := Item (Structure_Items, Index - 1);
      A := Get_Named_Item (Attributes (N), "delimiter");
      Put_Line ("Delimiter is """ & Value (A) & """ is "
             & Node_Value (First_Child (N)));
   end loop;

   Free (List);

   Free (Reader);
end ReadPrefs;
