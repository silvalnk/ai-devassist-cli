# frozen_string_literal: true

require "stub_llm"

class ResearchSkill
  def call(task)
    StubLLM.complete("Research the topic for this task:\n#{task}")
  end
end
