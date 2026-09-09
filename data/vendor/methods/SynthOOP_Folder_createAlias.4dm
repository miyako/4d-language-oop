// overload 0
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $destinationFolder1 : 4D.Folder
$destinationFolder1:=Folder(fk database folder)
var $result2 : 4D.File
$result2:=$receiver.createAlias($destinationFolder1; "synthText"; fk alias link)
// overload 0 [enum Folder.createAlias.aliasType = fk symbolic link]
$receiver:=Folder(fk database folder)
var $destinationFolder3 : 4D.Folder
$destinationFolder3:=Folder(fk database folder)
var $result4 : 4D.File
$result4:=$receiver.createAlias($destinationFolder3; "synthText"; fk symbolic link)
// overload 0 [optional:omit-aliasType]
$receiver:=Folder(fk database folder)
var $destinationFolder5 : 4D.Folder
$destinationFolder5:=Folder(fk database folder)
var $result6 : 4D.File
$result6:=$receiver.createAlias($destinationFolder5; "synthText")
