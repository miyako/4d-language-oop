// property read
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $read1 : 4D.Folder
$read1:=$receiver.currentDirectory
// property write
$receiver:=4D.SystemWorker.new("ls -l")
var $currentDirectory2 : 4D.Folder
$currentDirectory2:=Folder(fk database folder)
$receiver.currentDirectory:=$currentDirectory2
