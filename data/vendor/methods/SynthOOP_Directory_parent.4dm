// property read
// Directory is abstract; synthesized against its concrete subclass Folder.
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $read1 : 4D.Folder
$read1:=$receiver.parent
