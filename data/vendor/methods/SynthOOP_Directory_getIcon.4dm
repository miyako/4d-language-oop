// overload 0
// Directory is abstract; synthesized against its concrete subclass Folder.
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $result1 : Picture
$result1:=$receiver.getIcon(1)
// overload 0 [optional:omit-size]
// Directory is abstract; synthesized against its concrete subclass Folder.
$receiver:=Folder(fk database folder)
var $result2 : Picture
$result2:=$receiver.getIcon()
