# frozen_string_literal: true

require "open3"
require "json"

module Bridge
  module_function

  def repo_root
    File.expand_path("../..", __dir__)
  end

  def devassist_bin
    env = ENV["DEVASSIST_BIN"]
    return env if env && File.executable?(env)

    [
      File.join(repo_root, "core", "target", "debug", "devassist"),
      File.join(repo_root, "core", "target", "release", "devassist")
    ].find { |p| File.executable?(p) }
  end

  def ask_json(question)
    bin = devassist_bin
    unless bin
      return { "ok" => false, "error" => "devassist binary not built (cd core && cargo build)" }
    end

    store_dir = File.join(repo_root, "core")
    stdout, stderr, status = Open3.capture3(
      bin, "ask", "--json", question,
      chdir: store_dir
    )
    unless status.success?
      return { "ok" => false, "error" => stderr.strip.empty? ? stdout.strip : stderr.strip }
    end

    parsed = JSON.parse(stdout)
    hits = parsed.dig("result", "hits") || []
    { "ok" => true, "hits" => hits }
  rescue JSON::ParserError => e
    { "ok" => false, "error" => e.message }
  end
end
