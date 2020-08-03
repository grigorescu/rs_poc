#include "Plugin.h"

#include "analyzer/Component.h"

#include "syslog.h"

namespace plugin { namespace Demo_RustySyslog { Plugin plugin; } }

using namespace plugin::Demo_RustySyslog;

plugin::Configuration Plugin::Configure()
	{
	AddComponent(new ::analyzer::Component("RustySyslog",
	             ::analyzer::rustysyslog::RustySyslog_Analyzer::Instantiate));
	
	plugin::Configuration config;
	config.name = "Demo::RustySyslog";
	config.description = "<Insert description>";
	config.version.major = 0;
	config.version.minor = 1;
	config.version.patch = 0;
	return config;
	}
