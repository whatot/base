#!/usr/bin/python2
# -*- coding:utf-8 -*-

import smtplib
mail_server = 'localhost'
mail_server_port = 25
from_addr = 'sender@example.com'
to_addr = 'receiver@example.com'

from_header = 'From: %s\r\n' % from_addr
to_header = 'To: %s\r\n\r\n' % to_addr
subject_header = 'Subject: nothing intersting'

body = 'This is a not-very-interesting email.'

email_message = '%s\n%s\n%s\n\n%s' % (from_addr, to_addr, subject_header,
        body)

s = smtplib.SMTP(mail_server, mail_server_port)

#SMTP comfirm
s.set_debuglevel(1)
#start ssl
s.starttls()
s.login("fatalbert", "mypasswd")

s.sendmail(from_addr, to_addr, email_message)
s.quit()
