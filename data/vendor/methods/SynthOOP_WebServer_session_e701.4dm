// property read
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Text
$read1:=$receiver.sessionCookieSameSite
// property write
$receiver:=WEB Server
$receiver.sessionCookieSameSite:="synthText"
