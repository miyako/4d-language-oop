// property read
// Directory is abstract; synthesized against its concrete subclass Folder.
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $read1 : Date
$read1:=$receiver.creationDate
