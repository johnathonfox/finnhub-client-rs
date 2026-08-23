# Finnhub API coverage

Endpoint inventory captured from the official docs sidebar at
<https://finnhub.io/docs/api> on 2026-08-22. This is the implementation checklist
for `finnhub-client-rs`; check the box when the typed endpoint + model + test land.

**Tier labels** are Finnhub's own sidebar tags: `Premium` = paid plan required,
`New` = recently added, `High Usage` = heavily used by Finnhub customers.
No label = available on the free tier. The 30 requests/second ceiling applies to
all plans (see ADR-0004).

**Upstream doc paths** are relative to `https://finnhub.io/docs/api/`.

## WebSocket (out of scope for milestone 1 — see ADR-0005)

- [ ] Trades — `High Usage` — `websocket-trades`. 1 API key = 1 concurrent connection; UNIX **ms** timestamps; FXCM/Forex.com/FHFX not supported for streaming.
- [ ] News — `Premium` — `websocket-news`
- [ ] Press Releases — `Premium` — `websocket-press-releases`

## Stock fundamentals

- [x] Symbol Lookup — `symbol-search` (`misc().symbol_lookup`)
- [x] Stock Symbols — `stock-symbols` (`stock().symbols`)
- [x] Market Status — `New` — `market-status` (`stock().market_status`)
- [x] Market Holiday — `New` — `market-holiday` (`stock().market_holidays`)
- [ ] Company Profile — `Premium` — `company-profile`
- [x] Company Profile 2 — `New` — `company-profile2` (`stock().company_profile2`)
- [ ] Company Executive — `Premium` — `company-executive`
- [x] Market News — `market-news` (`news().market_news`)
- [x] Company News — `High Usage` — `company-news` (`news().company_news`)
- [ ] Press Releases — `Premium` — `press-releases`
- [ ] News Sentiment — `Premium` — `news-sentiment`
- [x] Peers — `company-peers` (`stock().peers`)
- [x] Basic Financials — `High Usage` — `company-basic-financials` (`stock().basic_financials`)
- [ ] Ownership — `Premium` — `ownership`
- [ ] Fund Ownership — `Premium` — `fund-ownership`
- [ ] Institutional Profile — `Premium` — `institutional-profile`
- [ ] Institutional Portfolio — `Premium` — `institutional-portfolio-13f`
- [ ] Institutional Ownership — `Premium` — `institutional-ownership`
- [ ] Insider Transactions — `New` — `insider-transactions`
- [ ] Insider Sentiment — `New` — `insider-sentiment`
- [ ] Financials — `Premium` — `financials`
- [ ] Financials As Reported — `New` — `financials-reported`
- [ ] Revenue Breakdown — `Premium` — `revenue-breakdown`
- [ ] SEC Filings — `New` — `filings`
- [ ] SEC Sentiment Analysis — `Premium` — `filings-sentiment`
- [ ] Similarity Index — `Premium` — `similarity-index`
- [x] IPO Calendar — `New` — `ipo-calendar` (`calendar().ipo`)
- [ ] Dividends — `Premium` — `stock-dividends`
- [ ] Sector Metrics — `Premium` — `sector-metrics`
- [ ] Price Metrics — `Premium` — `price-metrics`
- [ ] Symbol Change — `Premium` — `symbol-change`
- [ ] ISIN Change — `Premium` — `isin-change`
- [ ] Historical Market Cap — `Premium` — `historical-market-cap`
- [ ] Historical Employee — `Premium` — `historical-employee-count`

## Stock estimates

- [x] Recommendation Trends — `recommendation-trends` (`stock().recommendation_trends`)
- [ ] Price Target — `Premium` — `price-target`
- [ ] Upgrade/Downgrade — `Premium` — `upgrade-downgrade`
- [ ] Revenue Estimates — `Premium` — `company-revenue-estimates`
- [ ] EPS Estimates — `Premium` — `company-eps-estimates`
- [ ] EBITDA Estimates — `Premium` — `company-ebitda-estimates`
- [ ] EBIT Estimates — `Premium` — `company-ebit-estimates`
- [ ] Net Income Estimates — `Premium` — `company-net-income-estimates`
- [ ] Pretax Income Estimates — `Premium` — `company-pretax-income-estimates`
- [ ] Gross Income Estimates — `Premium` — `company-gross-income-estimates`
- [ ] DPS Estimates — `Premium` — `company-dps-estimates`
- [ ] FCF Estimates — `Premium` — `company-fcf-estimates`
- [ ] Capex Estimates — `Premium` — `company-capex-estimates`
- [ ] OCF Estimates — `Premium` — `company-ocf-estimates`
- [x] EPS Surprises — `High Usage` — `company-earnings` (`stock().eps_surprises`)
- [x] Earnings Calendar — `New` — `earnings-calendar` (`calendar().earnings`)

## Stock price

- [x] Quote — `High Usage` — `quote` (`stock().quote`)
- [ ] Candles (OHLCV) — `Premium` — `stock-candles`
- [ ] Tick/Trade Data — `Premium` — `stock-tick`
- [ ] Historical NBBO — `Premium` — `stock-nbbo`
- [ ] Last Bid-Ask — `Premium` — `stock-bidask`
- [ ] Splits — `Premium` — `stock-splits`
- [ ] Dividends 2 — `Premium` — `stock-basic-dividends`

## ETFs & indices — all `Premium`

- [ ] Indices Constituents — `indices-constituents`
- [ ] ETFs Profile — `etfs-profile`
- [ ] ETFs Holdings — `etfs-holdings`
- [ ] ETFs Sector — `etfs-sector-exposure`
- [ ] ETFs Country — `etfs-country-exposure`
- [ ] ETFs Allocation — `etfs-allocation`

## Mutual funds — all `Premium`

- [ ] Profile — `mutual-fund-profile` · Holdings — `mutual-fund-holdings` · Sector — `mutual-fund-sector-exposure` · Country — `mutual-fund-country-exposure` · EET — `mutual-fund-eet` · EET PAI — `mutual-fund-eet-pai`

## Bonds — all `Premium`

- [ ] Profile — `bond-profile` · Price — `bond-price` · Tick/Trade — `bond-tick` · Yield Curve — `bond-yield-curve`

## Forex

- [x] Exchanges — `forex-exchanges` (`forex().exchanges`)
- [x] Symbols — `forex-symbols` (`forex().symbols`)
- [ ] Candles — `Premium` — `forex-candles`
- [ ] All Rates — `Premium` — `forex-rates`

## Crypto

- [x] Exchanges — `crypto-exchanges` (`crypto().exchanges`)
- [x] Symbols — `crypto-symbols` (`crypto().symbols`)
- [ ] Profile — `Premium` — `crypto-profile`
- [ ] Candles — `Premium` — `crypto-candles`

## Technical analysis — all `Premium`

- [ ] Pattern Recognition — `pattern-recognition` · Support/Resistance — `support-resistance` · Aggregate Indicators — `aggregate-indicator` · Technical Indicators — `technical-indicator`

## Alternative data

- [ ] Transcripts List — `Premium` — `transcripts-list`
- [ ] Transcripts — `Premium` — `earnings-call-transcripts-api`
- [ ] Earnings Call Live — `Premium` — `earnings-call-live-api`
- [ ] Company Presentation — `Premium` — `stock-presentation-slide-api`
- [ ] Social Sentiment — `Premium` — `social-sentiment`
- [ ] Investment Themes — `Premium` — `investment-themes-thematic-investing`
- [ ] Supply Chain — `Premium` — `supply-chain-relationships`
- [ ] Company ESG — `Premium` — `company-esg-score-api`
- [ ] Historical ESG — `Premium` — `company-historical-esg-score-api`
- [ ] Earnings Quality Score — `Premium` — `company-earnings-quality-score-api`
- [ ] USPTO Patents — `New` — `stock-uspto-patent`
- [ ] Visa Application — `New` — `stock-visa-application`
- [ ] Senate Lobbying — `New` — `stock-lobbying`
- [ ] USA Spending — `New` — `stock-usa-spending`
- [ ] Congressional Trading — `Premium` — `congressional-trading`
- [ ] Bank Branch — `Premium` — `bank-branch-api`
- [x] FDA Calendar — `fda-committee-meeting-calendar` (`misc().fda_calendar`)

## Enterprise data — all `Premium`

- [ ] AI Copilot (POST) — `ai-copilot-llm` · Revenue Breakdown & KPI — `revenue-breakdown-kpi-api` · Newsroom — `stock-newsroom`

## Global filings search — all `Premium`

- [ ] International Filings — `international-filings` · Filings Search — `global-filings-search` · Search In Filing — `search-in-filing` · Search Filter — `global-filings-search-filter` · Download Filings — `global-filings-download`

## Economic

- [x] Country List — `country` (`misc().country`)
- [ ] Economic Calendar — `Premium` — `economic-calendar`
- [ ] Economic Codes — `Premium` — `economic-code`
- [ ] Economic Data — `Premium` — `economic-data`

## Milestone 1 — done

All 20 free-tier endpoints above are implemented and tested (wiremock; no live
network in tests). Verified once against the live API on 2026-08-22 via
`cargo run --example quote` (AAPL). Everything not yet typed is reachable through
the raw escape hatch: `client.raw().get(path, params)` (ADR-0005).
