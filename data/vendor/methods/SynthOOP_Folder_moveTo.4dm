// overload 0
var $receiver : 4D.Folder
$receiver:=Folder(fk database folder)
var $destinationFolder1 : 4D.Folder
$destinationFolder1:=Folder(fk database folder)
var $result2 : 4D.Folder
$result2:=$receiver.moveTo($destinationFolder1; "synthText")
// overload 0 [optional:omit-newName]
$receiver:=Folder(fk database folder)
var $destinationFolder3 : 4D.Folder
$destinationFolder3:=Folder(fk database folder)
var $result4 : 4D.Folder
$result4:=$receiver.moveTo($destinationFolder3)
