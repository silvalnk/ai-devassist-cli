# frozen_string_literal: true

require "bridge"

class RAGSkill
  def call(task)
    result = Bridge.ask_json(task)
    unless result["ok"]
      return "RAGSkill fallback (stub): #{result['error']}"
    end

    hits = result["hits"]
    return "RAGSkill: Not found." if hits.nil? || hits.empty?

    lines = hits.first(3).map do |h|
      text = h["text"].to_s.gsub(/\s+/, " ").slice(0, 120)
      "- #{h['score']} #{h['source']}: #{text}"
    end
    "RAGSkill hits:\n#{lines.join("\n")}"
  end
end
