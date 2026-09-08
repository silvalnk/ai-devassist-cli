# frozen_string_literal: true

require "json"
require "fileutils"
require "time"

class WorkflowEngine
  attr_reader :audit

  def initialize(name)
    @name = name
    @audit = []
    @aborted = false
  end

  def step(label)
    return if @aborted

    ok = yield
    record(label, ok)
    unless ok
      @aborted = true
      puts "[gate] stopped at #{label}"
    end
    ok
  end

  def record(label, ok)
    @audit << { "step" => label, "ok" => ok, "at" => Time.now.utc.iso8601 }
    puts "[audit] #{label}: #{ok ? 'ok' : 'fail'}"
  end

  def write_audit!(path)
    File.write(path, JSON.pretty_generate({ "workflow" => @name, "steps" => @audit }))
    puts "[audit] wrote #{path}"
  end
end
