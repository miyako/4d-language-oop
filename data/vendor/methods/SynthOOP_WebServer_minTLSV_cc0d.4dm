// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Integer
$read1:=$receiver.minTLSVersion
// property write
$receiver:=WEB Server
$receiver.minTLSVersion:=1
