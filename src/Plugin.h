
#ifndef BRO_PLUGIN_DEMO_RUSTYSYSLOG
#define BRO_PLUGIN_DEMO_RUSTYSYSLOG

#include <plugin/Plugin.h>

namespace plugin {
namespace Demo_RustySyslog {

class Plugin : public ::plugin::Plugin
{
protected:
	// Overridden from plugin::Plugin.
	plugin::Configuration Configure() override;
};

extern Plugin plugin;

}
}

#endif
