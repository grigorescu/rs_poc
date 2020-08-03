// See the file in the main distribution directory for copyright.

#pragma once

#include "analyzer/Analyzer.h"

namespace analyzer { namespace rustysyslog {

class RustySyslog_Analyzer : public analyzer::Analyzer {
public:
	explicit RustySyslog_Analyzer(Connection* conn)
	    : Analyzer("RustySyslog", conn)
		{}

	void DeliverPacket(int len, const u_char* data, bool orig,
	                   uint64_t seq, const IP_Hdr* ip, int caplen) override;

	static analyzer::Analyzer* Instantiate(Connection* conn)
		{ return new RustySyslog_Analyzer(conn); }
};

} } // namespace analyzer::*
