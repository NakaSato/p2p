# การพัฒนาระบบการซื้อขายพลังงานแสงอาทิตย์แบบ Peer-to-Peer ด้วยเทคโนโลยี Blockchain และ Smart Contracts: กรณีศึกษามหาวิทยาลัย

## หัวข้อโครงงาน (Project Title)
**"Development of Peer-to-Peer Solar Energy Trading System using Blockchain Technology and Smart Contracts: A University Campus Case Study"**

**หัวข้อภาษาไทย:** "การพัฒนาระบบการซื้อขายพลังงานแสงอาทิตย์แบบ Peer-to-Peer ด้วยเทคโนโลยี Blockchain และ Smart Contracts: กรณีศึกษามหาวิทยาลัย"

## ข้อมูลโปรเจค (Project Information)

### **สาขาวิชา:** วิศวกรรมคอมพิวเตอร์ / วิทยาการคอมพิวเตอร์
### **ประเภทโครงงาน:** การพัฒนาซอฟต์แวร์และระบบ Blockchain
### **คำสำคัญ:** Blockchain, Smart Contracts, P2P Energy Trading, Substrate, ink!, Solar Energy, IoT
### **ระดับความยาก:** สูง - ใช้เทคโนโลยีใหม่และซับซ้อน

## บทคัดย่อ (Abstract)

โปรเจคนี้เป็นการพัฒนาระบบการซื้อขายพลังงานแสงอาทิตย์แบบ Peer-to-Peer (P2P Energy Trading System) ที่ใช้เทคโนโลยี Blockchain และ Smart Contracts เพื่อให้ผู้ใช้ในมหาวิทยาลัยสามารถซื้อขายพลังงานแสงอาทิตย์ส่วนเกินได้โดยตรงกับกัน ผ่านระบบที่มีการจัดการแบบรวมศูนย์

### วัตถุประสงค์หลัก (Main Objectives)
1. **พัฒนาระบบ Blockchain**: สร้างระบบการซื้อขายพลังงานแบบ P2P บนเทคโนโลยี Blockchain
2. **สร้าง Smart Contracts**: พัฒนา Smart Contracts ด้วยภาษา ink! บน Substrate Framework
3. **ออกแบบ Token Economy**: สร้างระบบเศรษฐกิจ Token สำหรับการแลกเปลี่ยนพลังงาน (1 GRID = 1 kWh)
4. **พัฒนา Oracle System**: สร้างระบบ Oracle เพื่อเชื่อมต่อข้อมูลจาก Smart Meter Simulation
5. **สร้าง Full-Stack Application**: พัฒนา API Gateway และ Frontend สำหรับผู้ใช้งาน

### วัตถุประสงค์รอง (Secondary Objectives)
1. ศึกษาและเปรียบเทียบประสิทธิภาพของ Blockchain Frameworks
2. ออกแบบระบบที่สามารถขยายผลไปใช้ในมหาวิทยาลัยอื่น ๆ
3. พัฒนาต้นแบบที่สามารถนำไปใช้งานจริงได้
4. วิเคราะห์ผลกระทบทางเศรษฐกิจและสิ่งแวดล้อม

### เทคโนโลยีที่ใช้ (Technologies Used)
- **Blockchain**: Substrate Framework with ink! Smart Contracts
- **Backend**: Rust API Gateway with Axum framework
- **Frontend**: React TypeScript with Vite
- **Database**: PostgreSQL with TimescaleDB extension
- **Containerization**: Docker & Docker Compose
- **Token Standard**: PSP22 (Polkadot Standard Proposal 22)

### สถาปัตยกรรมระบบ (System Architecture)
ระบบประกอบด้วย 4 Smart Contracts หลัก และ Smart Meter Simulation:

1. **Registry Contract**: จัดการการลงทะเบียนผู้ใช้และ Smart Meters แบบรวมศูนย์
2. **GridToken Contract**: Token มาตรฐาน PSP22 สำหรับการซื้อขายพลังงาน
3. **Trading Contract**: ระบบตลาดการซื้อขายแบบ Order Book ที่มีการควบคุม
4. **Oracle Client Contract**: เชื่อมต่อข้อมูลจากภายนอกและทำ Market Clearing อัตโนมัติ
5. **Smart Meter Simulator**: จำลองการทำงานของ Smart Meter ส่งข้อมูลผ่าน API เข้า Blockchain

### คุณสมบัติหลัก (Key Features)
- การซื้อขายพลังงานแบบ Real-time ทุก 15 นาที
- ระบบ Automated Market Clearing ด้วย Oracle
- การรับรองตัวตนแบบ Centralized Identity Management
- ระบบ Token Economy: 1 kWh = 1 GRID Token
- Cross-Contract Communication
- RESTful API สำหรับ Integration
- Responsive Web Interface
- **Smart Meter Simulation**: การจำลองข้อมูลพลังงานแบบ Real-time
- **API-to-Blockchain Integration**: การส่งข้อมูลจากจำลอง Smart Meter เข้าสู่ Blockchain

### ผลลัพธ์ที่คาดหวัง (Expected Outcomes)
1. ระบบ P2P Energy Trading ที่ทำงานได้จริง
2. การลดต้นทุนในการซื้อขายพลังงาน
3. การเพิ่มประสิทธิภาพการใช้พลังงานหมุนเวียน
4. ต้นแบบสำหรับการนำไปใช้ในมหาวิทยาลัยอื่น ๆ
5. การศึกษาเปรียบเทียบ Blockchain Frameworks

### การประยุกต์ใช้ (Applications)
- มหาวิทยาลัยที่มีระบบ Solar Cells
- หมู่บ้านที่ใช้พลังงานหมุนเวียน
- โครงการ Smart City
- ระบบ Microgrid ในชุมชน

---

## Project: Development of Peer-to-Peer Solar Energy Trading System using Blockchain Technology and Smart Contracts: A University Campus Case Study

### Abstract

This project develops a peer-to-peer (P2P) energy trading system using blockchain technology and smart contracts, enabling university users to directly trade surplus solar energy through a centrally managed platform. The system leverages the Substrate framework with ink! smart contracts to create a transparent, secure, and efficient energy marketplace.

### Objectives
1. Develop a P2P energy trading system on blockchain technology
2. Create smart contracts using ink! language on Substrate framework
3. Design a token economy system for energy exchange (1 GRID = 1 kWh)
4. Develop an Oracle system to integrate Smart Meter data
5. Build API Gateway and Frontend for user interaction

### Technologies Used
- **Blockchain**: Substrate Framework with ink! Smart Contracts
- **Backend**: Rust API Gateway with Axum framework
- **Frontend**: React TypeScript with Vite
- **Database**: PostgreSQL with TimescaleDB extension
- **Containerization**: Docker & Docker Compose
- **Token Standard**: PSP22 (Polkadot Standard Proposal 22)

### System Architecture
The system consists of 4 core smart contracts and Smart Meter Simulation:

1. **Registry Contract**: Centrally manages user registration and Smart Meter assignments
2. **GridToken Contract**: PSP22 standard token for energy trading
3. **Trading Contract**: Controlled order book-based trading marketplace
4. **Oracle Client Contract**: External data integration and automated market clearing
5. **Smart Meter Simulator**: Simulates Smart Meter operations, sending data via API to Blockchain

### Key Features
- Real-time energy trading every 15 minutes
- Automated Market Clearing with Oracle integration
- Centralized Identity verification
- Token Economy: 1 kWh = 1 GRID Token
- Cross-Contract Communication
- RESTful API for integration
- Responsive Web Interface
- **Smart Meter Simulation**: Real-time energy data simulation
- **API-to-Blockchain Integration**: Data transmission from Smart Meter simulator to Blockchain

### Expected Outcomes
1. Functional P2P Energy Trading System
2. Reduced energy trading costs
3. Improved renewable energy utilization efficiency
4. Prototype for implementation in other universities
5. Comparative study of Blockchain Frameworks

### Applications
- Universities with Solar Cell systems
- Communities using renewable energy
- Smart City projects
- Community Microgrid systems

---

## ข้อมูลเพิ่มเติมสำหรับการนำเสนอ (Additional Information for Presentation)

### จุดเด่นของโปรเจค (Project Highlights)
1. **นวัตกรรม**: ใช้เทคโนโลยี Substrate ที่ทันสมัย
2. **ความปลอดภัย**: Smart Contracts ที่ผ่านการ Audit
3. **ประสิทธิภาพ**: ระบบ Real-time trading
4. **ความยั่งยืน**: ส่งเสริมการใช้พลังงานหมุนเวียน
5. **ความสามารถในการขยาย**: Modular architecture

### ความท้าทายที่แก้ไข (Challenges Addressed)
1. การขาดระบบการซื้อขายพลังงานที่โปร่งใส
2. ต้นทุนสูงในการซื้อขายผ่านตัวกลาง
3. การไม่มีระบบติดตามพลังงานหมุนเวียนแบบ Real-time
4. ปัญหาความน่าเชื่อถือในการซื้อขาย P2P

## รายละเอียดเทคนิค Smart Meter Simulation

### Smart Meter Simulator Architecture
ระบบจำลอง Smart Meter ประกอบด้วย:

#### 1. **Python Simulation Engine**
- จำลองการผลิตและใช้พลังงานแสงอาทิตย์
- สร้างข้อมูลแบบ Real-time ทุก 15 นาที
- จำลองสภาพอากาศและรูปแบบการใช้พลังงาน

#### 2. **API Integration Layer**
- **RESTful API**: ส่งข้อมูลจาก Simulator ไปยัง API Gateway
- **Data Format**: JSON format ที่มี meter_id, energy_generated, energy_consumed, timestamp
- **Authentication**: API key และ digital signature verification

#### 3. **Blockchain Data Flow**
```
Smart Meter Simulator → API Gateway → Oracle Client Contract → GridToken Contract
```

#### 4. **Data Processing Pipeline**
1. **Data Generation**: Simulator สร้างข้อมูลพลังงาน
2. **API Call**: POST ข้อมูลไปยัง `/api/meter-reading` endpoint
3. **Validation**: API Gateway ตรวจสอบความถูกต้องของข้อมูล
4. **Oracle Processing**: Oracle Client รับข้อมูลและประมวลผล
5. **Token Minting**: สร้าง GRID Token สำหรับพลังงานที่ผลิตได้
6. **Market Integration**: นำข้อมูลเข้าสู่ระบบการซื้อขาย

### ข้อมูลที่จำลอง (Simulated Data)

#### **Energy Generation Data**
- ข้อมูลการผลิตพลังงานจากแผงโซลาร์เซลล์
- รูปแบบการผลิตตามช่วงเวลา (เช้า-เย็น) แบบ Sine Curve
- ข้อมูลที่ปรับตามฤดูกาล และวันทำงาน/วันหยุด

#### **Energy Consumption Data**
- ข้อมูลการใช้พลังงานของอาคาร/หอพัก
- รูปแบบการใช้งานตามกิจกรรมของมหาวิทยาลัย
- ข้อมูลการใช้เครื่องปรับอากาศ เครื่องใช้ไฟฟ้า

#### **Smart Meter Metadata**
- Meter ID และ Location
- Owner Information (เชื่อมโยงกับ Registry Contract)
- Timestamp และ Digital Signature

## ประโยชน์และนวัตกรรม (Benefits & Innovation)

### **ประโยชน์ต่อมหาวิทยาลัย**
1. **ลดค่าไฟฟ้า**: การซื้อขายภายในลดการพึ่งพาระบบไฟฟ้าหลัก
2. **ส่งเสริมพลังงานสะอาด**: จูงใจให้ติดตั้งแผงโซลาร์เซลล์
3. **การศึกษาวิจัย**: เป็นแหล่งข้อมูลสำหรับการวิจัยต่อยอด
4. **ภาพลักษณ์**: แสดงความเป็นผู้นำด้านเทคโนโลยีสีเขียว
5. **การควบคุมและจัดการ**: มีระบบจัดการแบบรวมศูนย์ที่ปลอดภัยและเชื่อถือได้

### **ข้อดีของระบบแบบรวมศุนย์ (Centralized Approach Benefits)**
1. **ความปลอดภัย**: การควบคุมการเข้าถึงและการจัดการที่เข้มงวด
2. **การจัดการที่ง่าย**: มีจุดควบคุมหลักสำหรับการบริหารจัดการ
3. **ความเชื่อถือได้**: ระบบมีการตรวจสอบและควบคุมคุณภาพ
4. **การปฏิบัติตามกฎหมาย**: ง่ายต่อการปฏิบัติตามข้อกำหนดและกฎระเบียบ
5. **การสนับสนุนผู้ใช้**: มีจุดติดต่อหลักสำหรับการขอความช่วยเหลือ

### **นวัตกรรมทางเทคนิค**
1. **Substrate + ink!**: ใช้เทคโนโลยี Blockchain รุ่นใหม่
2. **Cross-Contract Communication**: Smart Contracts ที่ทำงานร่วมกัน
3. **Real-time Oracle Integration**: การเชื่อมต่อข้อมูลแบบเรียลไทม์
4. **PSP22 Token Standard**: มาตรฐาน Token ที่ทันสมัย
5. **Centralized Management**: ระบบจัดการแบบรวมศูนย์เพื่อความปลอดภัยและควบคุม

## ความเป็นไปได้และความเสี่ยง (Feasibility & Risks)

### **ความเป็นไปได้**
- ✅ เทคโนโลยีที่ใช้มีเอกสารและตัวอย่างเพียงพอ
- ✅ ทีมมีความรู้พื้นฐานด้าน Programming และ Blockchain
- ✅ มีเวลาเพียงพอสำหรับการพัฒนา (1-2 เทอม)
- ✅ สามารถทดสอบได้ด้วย Simulation

### **ความเสี่ยงและการแก้ไข**
1. **ความซับซ้อนของ Substrate**: แก้ไขด้วยการศึกษาเอกสารและ Tutorial
2. **การ Deploy Blockchain**: ใช้ Local Development Chain
3. **ปัญหา Performance**: ออกแบบระบบให้เหมาะสมกับการ Demo
4. **ข้อจำกัดเวลา**: แบ่งงานเป็น Milestone ที่ชัดเจน

## แผนการดำเนินงาน (Timeline)

### **Phase 1: การศึกษาและออกแบบ (4 สัปดาห์)**
- ศึกษา Substrate และ ink! Smart Contracts
- ออกแบบ System Architecture และ Database Schema
- สร้าง Project Setup และ Development Environment

### **Phase 2: การพัฒนา Smart Contracts (6 สัปดาห์)**
- พัฒนา Registry Contract
- พัฒนา GridToken Contract (PSP22)
- พัฒนา Trading Contract
- พัฒนา Oracle Client Contract
- Unit Testing และ Integration Testing

### **Phase 3: การพัฒนา Backend และ API (4 สัปดาห์)**
- สร้าง Rust API Gateway ด้วย Axum
- พัฒนา Smart Meter Simulator
- สร้าง Database Schema และ Data Models
- API Testing และ Documentation

### **Phase 4: การพัฒนา Frontend (4 สัปดาห์)**
- สร้าง React TypeScript Application
- ออกแบบ UI/UX สำหรับ Energy Trading
- สร้าง Dashboard สำหรับติดตามการซื้อขาย
- Responsive Design และ Testing

### **Phase 5: การรวมระบบและทดสอบ (3 สัปดาห์)**
- Integration Testing ทั้งระบบ
- Performance Testing
- Security Testing
- Bug Fixes และ Optimization

### **Phase 6: การเตรียมนำเสนอ (1 สัปดาห์)**
- สร้างเอกสารโครงงาน
- เตรียม Presentation และ Demo
- Video Demonstration

## เกณฑ์การประเมินผล (Evaluation Criteria)

### **เกณฑ์ทางเทคนิค (70%)**
1. **Smart Contracts (25%)**: ความถูกต้องและประสิทธิภาพ
2. **System Integration (20%)**: การทำงานร่วมกันของส่วนต่าง ๆ
3. **User Interface (15%)**: ความใช้งานง่ายและสวยงาม
4. **Code Quality (10%)**: การเขียนโค้ดที่ดี มี Documentation

### **เกณฑ์การนำเสนอ (30%)**
1. **การนำเสนอ (15%)**: ความชัดเจนและน่าสนใจ
2. **การตอบคำถาม (10%)**: ความเข้าใจในโครงงาน
3. **เอกสาร (5%)**: ความสมบูรณ์ของเอกสาร

### การพัฒนาในอนาคต (Future Development)
1. **Multi-chain Support**: การรองรับ Blockchain หลายเครือข่าย
2. **AI Price Prediction**: ระบบ AI สำหรับทำนายราคาพลังงาน
3. **National Grid Integration**: การเชื่อมต่อกับระบบไฟฟ้าหลัก
4. **Mobile Application**: แอปพลิเคชันมือถือ
5. **Real IoT Integration**: เชื่อมต่อกับ Smart Meter จริง
6. **Machine Learning Optimization**: ปรับปรุงประสิทธิภาพด้วย ML
7. **Carbon Credit Integration**: รวมระบบ Carbon Credit Trading
