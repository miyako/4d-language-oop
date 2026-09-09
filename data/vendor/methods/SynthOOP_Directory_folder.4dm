// overload 0
// Directory is abstract; synthesized against its concrete subclass Folder.
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $result1 : 4D.Folder
$result1:=$receiver.folder("synthText")
