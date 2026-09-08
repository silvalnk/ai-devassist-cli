# frozen_string_literal: true

require "skills/research_skill"
require "skills/code_gen_skill"
require "skills/rag_skill"

class Agent
  def initialize
    @skills = {
      "ResearchSkill" => ResearchSkill.new,
      "RAGSkill" => RAGSkill.new,
      "CodeGenSkill" => CodeGenSkill.new
    }
    @history = []
  end

  def run(task)
    thought = think(task)
    puts "[think] #{thought}"
    steps = plan(task)
    puts "[plan] #{steps.join(' → ')}"
    steps.each do |name|
      result = act(name, task)
      observation = observe(name, result)
      @history << observation
      puts "[observe] #{observation}"
    end
    puts "[done] #{@history.size} skill(s)"
  end

  def think(task)
    "Goal: #{task}. Pick skills from keywords (research / RAG / ADR)."
  end

  def plan(task)
    steps = []
    down = task.downcase
    # Learner README uses Portuguese verbs; also match English.
    steps << "ResearchSkill" if down.match?(/pesquis|research|busc/)
    steps << "RAGSkill" if down.match?(/rag|lua|docs|index|ask/)
    steps << "CodeGenSkill" if down.match?(/adr|gere|gerá|write|code|código|implement/)
    steps = @skills.keys if steps.empty?
    steps.uniq
  end

  def act(name, task)
    @skills.fetch(name).call(task)
  end

  def observe(name, result)
    preview = result.to_s.gsub(/\s+/, " ").slice(0, 160)
    "#{name}: #{preview}"
  end
end
