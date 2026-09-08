#!/usr/bin/env ruby
# frozen_string_literal: true

# CAP-5: RFC workflow with human-in-the-loop
$LOAD_PATH.unshift(File.expand_path("lib", __dir__))
require "workflows/rfc_workflow"

cmd = ARGV.shift
proposal = ARGV.join(" ")
if cmd != "rfc" || proposal.strip.empty?
  warn 'usage: ruby workflow.rb rfc "proposal"'
  exit 1
end

RfcWorkflow.new(proposal).run
