# GoldHunter

> Protocolo de Exploración Gamificada, Fidelización Comercial e Infraestructura de Marketing Cultural

---

## WHITE PAPER v1.2

### 1. RESUMEN EJECUTIVO

GoldHunter es una plataforma integral de **"Explore-to-Learn & Earn"** que conecta marcas y patrimonio cultural con la audiencia física mediante IA Generativa (Claude) y Web3 (Solana). El sistema transforma desplazamientos urbanos en experiencias de recompensa, permitiendo a las empresas adquirir "Packs de Cultura" para patrocinar el conocimiento y atraer tráfico cualificado a sus puntos de venta.

---

### 2. EL ECOSISTEMA TÉCNICO

El proyecto se divide en dos pilares fundamentales orquestados por el Jorch Lab:

#### A. GoldHunter App (B2C — El Usuario)

- **Exploración**: Mapa interactivo con Puntos de Interés (POI) Históricos y Comerciales.
- **Guardián Cognitivo (Claude AI)**: Validador multimodal que interactúa con el usuario, educa sobre el entorno y confirma la presencia física mediante desafíos dinámicos.
- **Álbum de Fragmentos**: Inventario de cNFTs (Solana) que representan hitos alcanzados.
- **Evolución Visual**: Sistema de Avatares que desbloquean "Skins" y accesorios (logros) tras completar rutas o canjes de marcas (ej. Termo Kurupí).

#### B. GoldHunter Business Hub (B2B — La Empresa)

Portal web para la gestión autónoma de campañas:

- **Registro de Marca**: Gestión de identidad visual y activos digitales.
- **Gestor de Rutas**: Panel para ubicar puntos comerciales en el mapa.
- **Marketplace de "Packs de Cultura"**: Adquisición de bibliotecas de conocimiento pre-configuradas (Rutas Jesuíticas, Historia del Deporte, Gastronomía Local) que la IA vincula automáticamente a los locales de la marca.

---

### 3. MECÁNICA DE INCENTIVOS (MERITOCRACIA DIGITAL)

| Paso | Acción | Resultado |
|------|--------|-----------|
| 1 | **Colección** | El usuario recolecta fragmentos al aprender y visitar lugares |
| 2 | **Escasez** | Álbumes de edición limitada — solo N cupos por mes por marca |
| 3 | **Hito de Finalización** | Al completar un álbum, se genera un NFT de Canje (Voucher) |
| 4 | **Canje en Mundo Real** | El usuario reclama premios físicos en el local del patrocinador |
| 5 | **Prueba de Honor** | Medalla digital permanente + skin exclusiva en el perfil |

---

### 4. MODELO DE NEGOCIO Y ESCALABILIDAD

- **SaaS para Marcas**: Suscripción por mantenimiento de puntos en el mapa y compra de Packs de Cultura.
- **CPA (Cost Per Action)**: Comisiones por cada canje de premio físico verificado por el sistema.
- **Expansión Futura**: Activación de "Airdrops Temporales" para eventos masivos (conciertos, lanzamientos) mediante la infraestructura ya desplegada.

---

### 5. STACK TECNOLÓGICO (JORCH LAB SETUP)

| Capa | Tecnología |
|------|-----------|
| **App B2C** | React Native + Expo (TypeScript) |
| **Business Hub B2B** | Next.js (TypeScript) |
| **Inteligencia** | Claude API (Anthropic) — Guardián Cognitivo |
| **Blockchain** | Solana + Anchor/Rust — cNFTs de alta eficiencia |
| **Automatización** | n8n + Scrapling — orquestación de flujos B2B/B2C |
| **MCP** | Servidor MCP propio — herramientas de POI, álbum y canje |
| **Frontend UI** | Shadcn/UI + Magic UI |
| **Seguridad** | Red mallada con Tailscale + Docker en VPS distribuidos |

---

### 6. PILLARES DE DISEÑO

1. **El Mundo Real es el Mapa** — Sin movimiento físico, no hay progreso.
2. **El Conocimiento es la Moneda** — El Guardián es el producto, no un obstáculo.
3. **El Logro debe ser Tangible** — Todo premio existe en el mundo real.
4. **La Escasez es el Respeto** — Los cupos limitados crean urgencia genuina.

---

### 7. SCOPE TIERS

| Tier | Contenido | Timeline |
|------|-----------|----------|
| **MVP** | 1 ciudad, 2 rutas, 15-20 POIs, 1 álbum, 1 patrocinador, canje QR manual | 8-10 semanas |
| **Vertical Slice** | 3 rutas, 30 POIs, 3 patrocinadores, Business Hub, cNFTs reales | 16-20 semanas |
| **Alpha** | 2 ciudades, 5 rutas, avatares, skins, airdrops temporales | 5-7 meses |
| **Full Vision** | Multi-ciudad, marketplace completo, API pública | 12-18 meses |

---

## Estructura del Repositorio

```
goldhunter-medo/
├── app/                    # React Native + Expo (B2C mobile app)
│   └── ...                 # Scaffolded with create-expo-app (TypeScript)
├── web/                    # Next.js (B2B Business Hub)
├── anchor/                 # Solana smart contracts (Rust + Anchor)
│   ├── programs/
│   │   └── goldhunter/
│   │       └── src/lib.rs  # Fragment mint + redeem logic
│   └── Anchor.toml
├── n8n/
│   └── workflows/          # Exported n8n workflow JSONs
├── mcp/
│   └── server/
│       └── config.json     # MCP tool definitions for Claude integration
├── docs/                   # Additional documentation
├── .gitignore
└── README.md               # Este archivo
```

---

## Quick Start

### App Mobile (B2C)
```bash
cd app
npm install
npx expo start
```

### Business Hub (B2B)
```bash
cd web
npm install
npm run dev
```

### Solana Smart Contracts
```bash
cd anchor
anchor build
anchor test
```

---

## Links

- Game Concept Document: `../Claude-Code-Game-Studios/design/gdd/game-concept.md`
- n8n Workflows: `./n8n/workflows/`
- MCP Server Config: `./mcp/server/config.json`

---

*Powered by Jorch Lab · Built with Claude Code · 2026*
