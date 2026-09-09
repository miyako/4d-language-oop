// property read
var $receiver : 4D.MailAttachment
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
var $read1 : Text
$read1:=$receiver.path
// property write
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
$receiver.path:="synthText"
