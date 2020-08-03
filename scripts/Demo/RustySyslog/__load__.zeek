event zeek_init()
	{
	Analyzer::register_for_ports(Analyzer::ANALYZER_RUSTYSYSLOG, set(514/udp));
	print "__load__.zeek: zeek_init() executed";
	}
