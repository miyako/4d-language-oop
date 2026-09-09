// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Collection
$read1:=$receiver.CORSSettings
// property write
$receiver:=WEB Server
$receiver.CORSSettings:=New collection
