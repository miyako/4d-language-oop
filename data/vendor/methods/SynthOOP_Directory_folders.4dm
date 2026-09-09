// overload 0
// Directory is abstract; synthesized against its concrete subclass Folder.
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $result1 : Collection
$result1:=$receiver.folders(fk recursive)
// overload 0 [enum Directory.folders.options = fk ignore invisible]
// Directory is abstract; synthesized against its concrete subclass Folder.
$receiver:=Folder(fk database folder)
var $result2 : Collection
$result2:=$receiver.folders(fk ignore invisible)
// overload 0 [optional:omit-options]
// Directory is abstract; synthesized against its concrete subclass Folder.
$receiver:=Folder(fk database folder)
var $result3 : Collection
$result3:=$receiver.folders()
