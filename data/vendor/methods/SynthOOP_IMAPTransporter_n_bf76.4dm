// property read
var $receiver : 4D.IMAPTransporter
$receiver:=4D.IMAPTransporter.new(New object("host"; "imap.example.com"))
var $read1 : 4D.IMAPNotifier
$read1:=$receiver.notifier
