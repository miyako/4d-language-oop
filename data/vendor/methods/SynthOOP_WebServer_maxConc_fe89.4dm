// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Integer
$read1:=$receiver.maxConcurrentProcesses
// property write
$receiver:=WEB Server
$receiver.maxConcurrentProcesses:=1
