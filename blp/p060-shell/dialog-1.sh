#!/bin/sh

# ask some questions and collect the answer

dialog --title "Questionmarire" --msgbox "Welcome to my simple survey" 9 18

dialog --title "Comfirm" --yesno "Are you willing to take part?" 9 18
if [ $? != 0 ]; then
	dialog --infobox "Thank you anyway" 5 20
	sleep 2
	dialog --clear
	exit 0
fi

dialog --title "Questionmarire" --inputbox "please enter your name" 9 30 2>_1.txt
Q_NAME=$(cat _1.txt)

dialog --menu "$Q_NAME, what music do you like best?" 15 30 4 1 "classial" 2 "Jazz" 3 "Country" 4 "Other" 2>_1.txt
Q_MUSIC=$(cat _1.txt)

if [ "$Q_MUSIC" == "1" ]; then
	dialog --msgbox "Good choice" 12 25
fi

sleep 3
dialog --clear
exit 0
