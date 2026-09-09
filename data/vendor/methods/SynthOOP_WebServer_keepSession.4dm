// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Boolean
$read1:=$receiver.keepSession
// property write
$receiver:=WEB Server
$receiver.keepSession:=True
