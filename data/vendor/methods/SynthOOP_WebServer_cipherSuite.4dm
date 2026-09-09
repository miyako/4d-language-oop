// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Text
$read1:=$receiver.cipherSuite
// property write
$receiver:=WEB Server
$receiver.cipherSuite:="synthText"
