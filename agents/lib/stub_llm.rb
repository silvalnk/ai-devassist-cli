# frozen_string_literal: true

# Deterministic stand-in for a chat model (no API key).
module StubLLM
  def self.complete(prompt)
    if prompt.start_with?("Write an ADR")
      <<~ADR
        # ADR stub (StubLLM)

        - Status: Proposed
        - Context: #{prompt.lines.first.to_s.strip}
        - Decision: document the change as an ADR before coding.
        - Consequences: learners see Context → Decision → Consequences.
      ADR
    elsif prompt.start_with?("Research")
      "Research notes (stub): RAG = retrieve chunks, then generate. In DevAssist, embeddings are a hash stub. Task: #{prompt.lines.last}"
    else
      "Stub completion:\n#{prompt.lines.first}"
    end
  end
end
