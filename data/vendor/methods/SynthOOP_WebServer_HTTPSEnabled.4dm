// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Boolean
$read1:=$receiver.HTTPSEnabled
// property write
$receiver:=WEB Server
$receiver.HTTPSEnabled:=True
