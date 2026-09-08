#!/usr/bin/env ruby
# frozen_string_literal: true

# CAP-4: Agent loop think → plan → act → observe
$LOAD_PATH.unshift(File.expand_path("lib", __dir__))
require "agent"

task = ARGV.join(" ")
if task.strip.empty?
  warn 'usage: ruby run.rb "research RAG and write an ADR"'
  exit 1
end

Agent.new.run(task)
