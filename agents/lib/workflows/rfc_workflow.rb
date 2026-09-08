# frozen_string_literal: true

require "fileutils"
require "workflows/workflow_engine"

class RfcWorkflow
  def initialize(proposal)
    @proposal = proposal
    @root = File.expand_path("../../..", __dir__)
    @engine = WorkflowEngine.new("rfc")
  end

  def run
    path = nil
    @engine.step("RFC") { path = write_rfc; true }
    @engine.step("Review") { puts File.read(path); true }
    approved = false
    @engine.step("Approve") { approved = hitl_approve?; approved }
    @engine.step("Implement") { puts "[implement] stub: would apply #{@proposal}"; true } if approved
    @engine.step("Verify") { puts "[verify] stub: cargo test would run here"; true } if approved
    audit_path = File.join(File.dirname(path), "AUDIT.json")
    @engine.write_audit!(audit_path)
    puts approved ? "[rfc] approved" : "[rfc] rejected"
  end

  def write_rfc
    dir = File.join(@root, "docs", "rfcs")
    FileUtils.mkdir_p(dir)
    slug = @proposal.downcase.gsub(/[^a-z0-9]+/, "-").gsub(/^-|-$/, "")
    slug = "proposal" if slug.empty?
    n = Dir.glob(File.join(dir, "*.md")).grep(%r{/\d+-}).size + 1
    path = File.join(dir, format("%03d-%s.md", n, slug))
    template = File.read(File.join(dir, "TEMPLATE.md"))
    body = template
           .gsub("<título>", @proposal)
           .gsub("<data>", Time.now.utc.strftime("%Y-%m-%d"))
           .gsub("<um parágrafo>", @proposal)
    File.write(path, body.sub("Rascunho", "Revisão"))
    puts "[rfc] wrote #{path}"
    path
  end

  def hitl_approve?
    print "Approve? [y/n] "
    $stdout.flush
    line = STDIN.gets
    line.to_s.strip.downcase.start_with?("y")
  end
end
