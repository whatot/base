buildjar:
	mvn clean compile assembly:single

# http://stackoverflow.com/questions/574594/how-can-i-create-an-executable-jar-with-dependencies-using-maven

clean:
	rm -rf target/
	rm -rf logs/*.hprof
	rm -rf hs*.log

distclean:
	for dir in `find . -maxdepth 1 -type d -print`; \
	do \
		make -C "$${dir}" clean; \
	done

.PHONY: all buildjar clean distclean
