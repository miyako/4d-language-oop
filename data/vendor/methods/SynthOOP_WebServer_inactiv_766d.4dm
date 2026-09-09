// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Integer
$read1:=$receiver.inactiveProcessTimeout
// property write
$receiver:=WEB Server
$receiver.inactiveProcessTimeout:=1
