buildjar:
	mvn clean compile assembly:single

clean:
	rm -rf target/
	rm -rf logs/*.hprof
	rm -rf hs*.log

# http://stackoverflow.com/questions/574594/how-can-i-create-an-executable-jar-with-dependencies-using-maven

.PHONY: all buildjar clean
