# ADR 005 — Defesas de prompt injection no tempo do stub

- Status: Aceito
- Data: 2026-09-08

## Contexto

O CAP-7 precisa de um eval de segurança sem LLM real. Os ataques ainda parecem injection e jailbreak de produção.

## Decisão

1. `sanitize_input` bloqueia frases conhecidas de override de instrução e persona DAN **antes** de qualquer geração stub.
2. `validate_output` marca texto que vaza “system prompt” / persona DAN.
3. `devassist security audit` roda payloads de `tests/security/PAYLOADS.txt`.

## Consequências

- As defesas são **allow/deny por palavra-chave**, não um firewall de modelo.
- A lição é o loop de eval (payload → sanitize → validate → pass/fail).
- Ligar um LLM de verdade depois mantém o mesmo comando de audit.

## Alternativas consideradas

- Só documentar OWASP — perderia o sinal de sucesso do CAP-7.
- Chamar API paga de moderação — fora da spec.
