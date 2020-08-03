# @TEST-EXEC: zeek -NN Demo::RustySyslog |sed -e 's/version.*)/version)/g' >output
# @TEST-EXEC: btest-diff output
