// overload 0
// Document is abstract; synthesized against its concrete subclass File.
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $destinationFolder1 : 4D.Folder
$destinationFolder1:=Folder(fk database folder)
var $result2 : 4D.File
$result2:=$receiver.copyTo($destinationFolder1; "synthText"; fk overwrite)
// overload 0 [optional:omit-newName]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $destinationFolder3 : 4D.Folder
$destinationFolder3:=Folder(fk database folder)
var $result4 : 4D.File
$result4:=$receiver.copyTo($destinationFolder3)
// overload 0 [optional:omit-overwrite]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $destinationFolder5 : 4D.Folder
$destinationFolder5:=Folder(fk database folder)
var $result6 : 4D.File
$result6:=$receiver.copyTo($destinationFolder5; "synthText")
