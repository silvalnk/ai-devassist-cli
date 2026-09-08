# frozen_string_literal: true

require "stub_llm"

class CodeGenSkill
  def call(task)
    StubLLM.complete("Write an ADR for:\n#{task}")
  end
end
