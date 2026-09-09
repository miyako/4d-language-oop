// property read
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $read1 : Integer
$read1:=$receiver.checkConnectionDelay
// property write
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
$receiver.checkConnectionDelay:=1
