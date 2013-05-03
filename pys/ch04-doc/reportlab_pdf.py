# -*- coding:utf-8 -*-
#!/usr/bin/python2

import subprocess
import datetime
from reportlab.pdfgen import canvas
from reportlab.lib.units import inch

import reportlab.pdfbase.ttfonts
reportlab.pdfbase.pdfmetrics.registerFont(reportlab.pdfbase.ttfonts.TTFont('wqyzh', '/usr/share/fonts/wenquanyi/wqy-zenhei/wqy-zenhei.ttc'))
import reportlab.lib.fonts


def disk_report():
    p = subprocess.Popen("df -h", shell=True, stdout=subprocess.PIPE)
    return p.stdout.readlines()

def create_pdf(input,output="disk_report.pdf"):
    now = datetime.datetime.today()
    date = now.strftime("%h %d %Y %H:%M:%S")
    c = canvas.Canvas(output)
    c.setFont("wqyzh",12)
    textobject = c.beginText()
    textobject.setTextOrigin(inch, 11*inch)
    textobject.textLines('''
    Disk Capacity Report: %s
    ''' % date)
    for line in input:
        textobject.textLines(line.strip())
    c.drawText(textobject)
    c.showPage()
    c.save()

report = disk_report()
create_pdf(report)
