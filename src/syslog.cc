#include "syslog.h"
#include "parser/parser.hpp"

using namespace analyzer::rustysyslog;

void RustySyslog_Analyzer::DeliverPacket(int len, const u_char* data, bool orig,
                                   uint64_t seq, const IP_Hdr* ip, int caplen)
	{
		ffi::rust_function(len);
		printf("C++: Received a packet of %d bytes.", len);
	}
