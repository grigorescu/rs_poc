#include "Conn.h"
#include "event.h"
#include "Val.h"

#include "syslog.h"
#include "parser/parser.hpp"

extern EventHandlerPtr syslog_message;

using namespace analyzer::rustysyslog;

void RustySyslog_Analyzer::DeliverPacket(int len, const u_char* data, bool orig,
                                   uint64_t seq, const IP_Hdr* ip, int caplen)
	{
		ffi::SyslogFields result = ffi::parse(len, data, orig, seq, ip, caplen);
		if ( ! result.success )
			{
			ProtocolViolation((const char*) result.error.string, (const char*) data, result.error.length);
			}

		if ( syslog_message )
			this->ConnectionEventFast(syslog_message, {
				this->BuildConnVal(),
				val_mgr->GetCount(result.priority.facility),
				val_mgr->GetCount(result.priority.severity),
				new StringVal(result.msg.length, (const char *)result.msg.string)
				});

	}
