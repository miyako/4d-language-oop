// overload 0
// Directory is abstract; synthesized against its concrete subclass Folder.
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $destinationFolder1 : 4D.Folder
$destinationFolder1:=Folder(fk database folder)
var $result2 : 4D.Folder
$result2:=$receiver.copyTo($destinationFolder1; "synthText"; fk overwrite)
// overload 0 [optional:omit-newName]
// Directory is abstract; synthesized against its concrete subclass Folder.
$receiver:=Folder(fk database folder)
var $destinationFolder3 : 4D.Folder
$destinationFolder3:=Folder(fk database folder)
var $result4 : 4D.Folder
$result4:=$receiver.copyTo($destinationFolder3)
// overload 0 [optional:omit-overwrite]
// Directory is abstract; synthesized against its concrete subclass Folder.
$receiver:=Folder(fk database folder)
var $destinationFolder5 : 4D.Folder
$destinationFolder5:=Folder(fk database folder)
var $result6 : 4D.Folder
$result6:=$receiver.copyTo($destinationFolder5; "synthText")
