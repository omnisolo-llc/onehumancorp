pub const SETUP_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>OneHuman - Setup Wizard</title>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg-dark: #0f172a;
            --glass-bg: rgba(255, 255, 255, 0.03);
            --glass-border: rgba(255, 255, 255, 0.1);
            --glass-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
            --primary: #4ecca3;
            --primary-hover: #45b38e;
            --text-main: #ffffff;
            --text-dim: rgba(255, 255, 255, 0.7);
            --danger: #ef4444;
            --transition-speed: 0.3s;
            --cubic-ease: cubic-bezier(0.4, 0, 0.2, 1);
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Outfit', sans-serif;
        }

        body {
            background: linear-gradient(135deg, #1a1a2e, #16213e);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            overflow-x: hidden;
            -webkit-font-smoothing: antialiased;
        }

        /* Glassmorphism Navigation */
        nav {
            position: fixed;
            top: 0;
            width: 100%;
            padding: 20px;
            display: flex;
            gap: 20px;
            backdrop-filter: blur(15px);
            background: rgba(255, 255, 255, 0.05);
            z-index: 1000;
            border-bottom: 1px solid var(--glass-border);
        }

        nav a {
            color: var(--text-main);
            text-decoration: none;
            font-weight: 500;
            opacity: 0.8;
            transition: opacity var(--transition-speed);
        }

        nav a:hover {
            opacity: 1;
        }

        nav a.active {
            color: var(--primary);
            opacity: 1;
        }

        /* Layout Container */
        .app-container {
            flex: 1;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 80px 20px 20px;
            width: 100%;
            max-width: 600px;
            margin: 0 auto;
        }

        /* Glassmorphism Card */
        .glass-card {
            background: var(--glass-bg);
            backdrop-filter: blur(20px);
            border-radius: 24px;
            padding: 40px;
            width: 100%;
            border: 1px solid var(--glass-border);
            box-shadow: var(--glass-shadow);
            transition: transform var(--transition-speed) var(--cubic-ease), opacity var(--transition-speed) var(--cubic-ease);
        }

        /* Typography */
        h1 {
            font-size: 28px;
            font-weight: 600;
            margin-bottom: 8px;
            text-align: center;
        }

        h2 {
            font-size: 24px;
            font-weight: 600;
            margin-bottom: 16px;
            text-align: center;
            color: var(--primary);
        }

        .subtitle {
            text-align: center;
            color: var(--text-dim);
            margin-bottom: 32px;
            font-size: 16px;
            line-height: 1.5;
        }

        /* Form Elements */
        .input-group {
            margin-bottom: 24px;
            width: 100%;
        }

        .input-label {
            display: block;
            margin-bottom: 8px;
            font-size: 14px;
            font-weight: 500;
            color: var(--text-dim);
        }

        input[type="text"],
        input[type="email"],
        input[type="password"],
        input[type="number"],
        textarea {
            width: 100%;
            padding: 16px;
            border-radius: 12px;
            border: 1px solid var(--glass-border);
            background: rgba(255, 255, 255, 0.05);
            color: var(--text-main);
            font-size: 16px;
            transition: all 0.2s ease;
        }

        input:focus,
        textarea:focus {
            outline: none;
            border-color: var(--primary);
            background: rgba(255, 255, 255, 0.08);
            box-shadow: 0 0 0 4px rgba(78, 204, 163, 0.1);
        }

        /* Selection Cards */
        .selection-grid {
            display: grid;
            grid-template-columns: 1fr;
            gap: 16px;
            margin-bottom: 32px;
        }

        @media (min-width: 480px) {
            .selection-grid {
                grid-template-columns: 1fr 1fr;
            }
        }

        .selection-card {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--glass-border);
            border-radius: 16px;
            padding: 20px;
            cursor: pointer;
            transition: all 0.2s ease;
            text-align: center;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 120px;
        }

        .selection-card:hover {
            background: rgba(255, 255, 255, 0.08);
            transform: translateY(-2px);
        }

        .selection-card.selected {
            background: rgba(78, 204, 163, 0.1);
            border-color: var(--primary);
        }

        .card-icon {
            font-size: 32px;
            margin-bottom: 12px;
        }

        .card-title {
            font-weight: 500;
            font-size: 16px;
        }

        /* Buttons */
        .btn {
            width: 100%;
            padding: 16px 24px;
            border-radius: 12px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s ease;
            border: none;
            display: flex;
            justify-content: center;
            align-items: center;
            gap: 8px;
        }

        .btn-primary {
            background: var(--primary);
            color: var(--bg-dark);
        }

        .btn-primary:hover {
            background: var(--primary-hover);
            transform: translateY(-1px);
        }

        .btn-secondary {
            background: rgba(255, 255, 255, 0.1);
            color: var(--text-main);
            border: 1px solid var(--glass-border);
        }

        .btn-secondary:hover {
            background: rgba(255, 255, 255, 0.15);
        }

        .btn-magic {
            background: linear-gradient(45deg, #a855f7, #ec4899);
            color: white;
            position: relative;
            overflow: hidden;
        }

        .btn-magic::after {
            content: '';
            position: absolute;
            top: 0;
            left: -100%;
            width: 50%;
            height: 100%;
            background: linear-gradient(to right, transparent, rgba(255,255,255,0.3), transparent);
            transform: skewX(-20deg);
            animation: shine 3s infinite;
        }

        @keyframes shine {
            0% { left: -100%; }
            20% { left: 200%; }
            100% { left: 200%; }
        }

        /* Progress Bar */
        .progress-container {
            width: 100%;
            height: 6px;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 3px;
            margin-bottom: 32px;
            overflow: hidden;
        }

        .progress-bar {
            height: 100%;
            background: var(--primary);
            border-radius: 3px;
            transition: width var(--transition-speed) var(--cubic-ease);
        }

        /* Loading Spinner */
        .spinner {
            width: 24px;
            height: 24px;
            border: 3px solid rgba(255,255,255,0.3);
            border-radius: 50%;
            border-top-color: var(--text-main);
            animation: spin 1s ease-in-out infinite;
            display: none;
        }

        .spinner.active {
            display: inline-block;
        }

        @keyframes spin {
            to { transform: rotate(360deg); }
        }

        /* Animations */
        .step-container {
            animation: fadeIn 0.3s var(--cubic-ease);
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }

        /* Checklist */
        .checklist {
            list-style: none;
            text-align: left;
            background: rgba(0,0,0,0.2);
            padding: 24px;
            border-radius: 16px;
            margin-bottom: 24px;
        }

        .checklist li {
            margin-bottom: 12px;
            display: flex;
            align-items: center;
            gap: 12px;
            font-size: 16px;
        }

        .checklist li:last-child {
            margin-bottom: 0;
        }

        .check-icon {
            font-size: 20px;
        }

        /* Photo Upload */
        .photo-upload {
            border: 2px dashed var(--glass-border);
            border-radius: 16px;
            padding: 32px;
            text-align: center;
            cursor: pointer;
            transition: all 0.2s ease;
            background: rgba(255,255,255,0.02);
            margin-bottom: 24px;
        }

        .photo-upload:hover {
            border-color: var(--primary);
            background: rgba(78, 204, 163, 0.05);
        }

        .photo-upload.has-file {
            border-style: solid;
            border-color: var(--primary);
            padding: 16px;
        }

        .photo-preview {
            max-width: 100%;
            max-height: 200px;
            border-radius: 8px;
            display: none;
        }

        /* Live Preview Mini */
        .live-preview {
            background: #ffffff;
            color: #333;
            border-radius: 12px;
            padding: 16px;
            margin-bottom: 24px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.2);
            text-align: left;
            position: relative;
            overflow: hidden;
            display: none;
        }

        .live-preview-header {
            font-weight: bold;
            font-size: 18px;
            margin-bottom: 8px;
            border-bottom: 1px solid #eee;
            padding-bottom: 8px;
        }

        .live-preview-body {
            font-size: 14px;
            color: #666;
        }

        .live-preview-btn {
            background: var(--primary);
            color: white;
            padding: 6px 12px;
            border-radius: 4px;
            font-size: 12px;
            display: inline-block;
            margin-top: 12px;
        }

        /* Mobile Adjustments */
        @media (max-width: 480px) {
            .glass-card {
                padding: 24px;
                border-radius: 16px;
            }
            h1 { font-size: 24px; }
            h2 { font-size: 20px; }
        }

        /* Hidden Utility */
        .hidden { display: none !important; }

        /* Confetti Canvas */
        #confetti {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            pointer-events: none;
            z-index: 9999;
        }
    </style>
</head>
<body>
    <nav>
        <a href="/">Dashboard</a>
        <a href="/agents">Agents</a>
    </nav>

    <canvas id="confetti"></canvas>

    <div class="app-container">
        <div class="glass-card" id="wizard-card">

            <div class="progress-container" id="progress-container">
                <div class="progress-bar" id="progress-bar" style="width: 0%;"></div>
            </div>

            <!-- Step 0: Welcome -->
            <div id="step-0" class="step-container">
                <h1>Setup Wizard</h1>
                <p class="subtitle">Your business, live in minutes.</p>
                <button class="btn btn-primary" onclick="nextStep()">Start Setup</button>
            </div>

            <!-- Step 1: Business Type -->
            <div id="step-1" class="step-container hidden">
                <h2>What type of business are you building?</h2>
                <p class="subtitle">This helps us tailor your experience.</p>

                <div class="selection-grid">
                    <div class="selection-card" onclick="selectOption('businessType', 'Online Store', this)">
                        <div class="card-icon">🛍️</div>
                        <div class="card-title">Online Store</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('businessType', 'Restaurant / Food', this)">
                        <div class="card-icon">🍽️</div>
                        <div class="card-title">Restaurant / Food</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('businessType', 'Services', this)">
                        <div class="card-icon">✂️</div>
                        <div class="card-title">Services</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('businessType', 'Other', this)">
                        <div class="card-icon">✨</div>
                        <div class="card-title">Other</div>
                    </div>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" id="btn-next-1" onclick="nextStep()" disabled>Next</button>
                </div>
            </div>

            <!-- Step 2: Company Name -->
            <div id="step-2" class="step-container hidden">
                <h2>What is your business called?</h2>
                <p class="subtitle">Don't worry, you can change this later.</p>

                <div class="input-group">
                    <input type="text" id="companyName" placeholder="e.g. Maya's Bakes" oninput="updateState('companyName', this.value); checkRequired('companyName', 'btn-next-2')">
                </div>

                <div class="input-group">
                    <label class="input-label">Business Description</label>
                    <textarea id="companyDescription" rows="3" placeholder="Briefly describe what you do..." oninput="updateState('companyDescription', this.value)"></textarea>
                    <button class="btn btn-magic" style="margin-top: 12px; padding: 12px;" onclick="generateAiDescription('companyName', 'companyDescription')">
                        <span class="btn-text">Generate Description</span>
                        <div class="spinner"></div>
                    </button>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" id="btn-next-2" onclick="nextStep()" disabled>Next</button>
                </div>
            </div>

            <!-- Step 3: Selling Categories -->
            <div id="step-3" class="step-container hidden">
                <h2>What will you be selling?</h2>
                <p class="subtitle">Select all that apply.</p>

                <div style="text-align: left; margin-bottom: 24px;">
                    <label style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px; cursor: pointer; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;">
                        <input type="checkbox" value="Physical Products" onchange="toggleArrayOption('categories', this.value)" style="width: 20px; height: 20px;">
                        <span>Physical Products</span>
                    </label>
                    <label style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px; cursor: pointer; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;">
                        <input type="checkbox" value="Digital Products" onchange="toggleArrayOption('categories', this.value)" style="width: 20px; height: 20px;">
                        <span>Digital Products</span>
                    </label>
                    <label style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px; cursor: pointer; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;">
                        <input type="checkbox" value="Services / Bookings" onchange="toggleArrayOption('categories', this.value)" style="width: 20px; height: 20px;">
                        <span>Services / Bookings</span>
                    </label>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" onclick="nextStep()">Next</button>
                </div>
            </div>

            <!-- Step 4: First Product -->
            <div id="step-4" class="step-container hidden">
                <h2>Let's add your first item</h2>
                <p class="subtitle">You can add more later.</p>

                <div class="photo-upload" id="photo-upload-zone" onclick="document.getElementById('productImage').click()">
                    <div id="upload-prompt">
                        <div class="card-icon">📸</div>
                        <div>Tap to upload a photo</div>
                    </div>
                    <img id="photo-preview" class="photo-preview" src="" alt="Preview">
                    <input type="file" id="productImage" accept="image/*" style="display: none" onchange="handlePhotoUpload(this)">
                </div>

                <div class="input-group">
                    <input type="text" id="productName" placeholder="What is the name of this product?" oninput="updateState('productName', this.value); checkRequired('productName', 'btn-next-4')">
                </div>

                <div class="input-group" style="display: flex; gap: 12px; align-items: center;">
                    <span style="font-size: 20px; color: var(--text-dim);">$</span>
                    <input type="number" id="productPrice" placeholder="0.00" step="0.01" oninput="updateState('productPrice', this.value)" style="flex: 1;">
                </div>

                <div class="input-group">
                    <textarea id="productDescription" rows="2" placeholder="Description..." oninput="updateState('productDescription', this.value)"></textarea>
                    <button class="btn btn-magic" style="margin-top: 12px; padding: 8px;" onclick="generateAiDescription('productName', 'productDescription')">
                        <span class="btn-text">Generate AI Description</span>
                        <div class="spinner"></div>
                    </button>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" id="btn-next-4" onclick="nextStep()" disabled>Next</button>
                </div>
            </div>

            <!-- Step 5: Payments -->
            <div id="step-5" class="step-container hidden">
                <h2>How do you want to get paid?</h2>
                <p class="subtitle">Choose your preferred method.</p>

                <div class="selection-grid">
                    <div class="selection-card" onclick="selectOption('paymentMethod', 'Online', this)">
                        <div class="card-icon">💳</div>
                        <div class="card-title">Online</div>
                        <div style="font-size: 12px; color: var(--text-dim); margin-top: 8px;">Credit Cards, Apple Pay</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('paymentMethod', 'In-Person', this)">
                        <div class="card-icon">📱</div>
                        <div class="card-title">In-Person</div>
                        <div style="font-size: 12px; color: var(--text-dim); margin-top: 8px;">Cash, POS</div>
                    </div>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" id="btn-next-5" onclick="nextStep()" disabled>Next</button>
                </div>
            </div>

            <!-- Step 6: Template -->
            <div id="step-6" class="step-container hidden">
                <h2>Choose a vibe</h2>
                <p class="subtitle">Select a template for your store.</p>

                <div class="live-preview" id="live-preview-box">
                    <div class="live-preview-header" id="lp-title">Your Store Name</div>
                    <div class="live-preview-body">
                        <p id="lp-desc">Welcome to our store. We offer the best products.</p>
                        <div class="live-preview-btn">Shop Now</div>
                    </div>
                </div>

                <div class="selection-grid" style="grid-template-columns: 1fr 1fr 1fr;">
                    <div class="selection-card" onclick="selectOption('theme', 'Modern', this); updateLivePreview('Modern')" style="min-height: 80px; padding: 12px;">
                        <div class="card-title">Modern</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('theme', 'Classic', this); updateLivePreview('Classic')" style="min-height: 80px; padding: 12px;">
                        <div class="card-title">Classic</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('theme', 'Playful', this); updateLivePreview('Playful')" style="min-height: 80px; padding: 12px;">
                        <div class="card-title">Playful</div>
                    </div>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" id="btn-next-6" onclick="nextStep()" disabled>Next</button>
                </div>
            </div>

            <!-- Step 7: Domain -->
            <div id="step-7" class="step-container hidden">
                <h2>Claim your web address</h2>
                <p class="subtitle">Where will customers find you?</p>

                <div class="selection-grid">
                    <div class="selection-card" onclick="selectOption('domainType', 'Free', this)">
                        <div class="card-icon">🌐</div>
                        <div class="card-title">Free OHC Domain</div>
                        <div id="suggested-domain" style="font-size: 12px; color: var(--primary); margin-top: 8px; word-break: break-all;">yourbusiness.ohc.app</div>
                    </div>
                    <div class="selection-card" onclick="selectOption('domainType', 'Custom', this)">
                        <div class="card-icon">🔗</div>
                        <div class="card-title">Connect Existing</div>
                        <div style="font-size: 12px; color: var(--text-dim); margin-top: 8px;">I already own a domain</div>
                    </div>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-primary" id="btn-next-7" onclick="nextStep()" disabled>Next</button>
                </div>
            </div>

            <!-- Step 8: Review & Launch -->
            <div id="step-8" class="step-container hidden">
                <h2>Ready to launch?</h2>
                <p class="subtitle">Review your setup and publish.</p>

                <div style="background: rgba(0,0,0,0.2); padding: 20px; border-radius: 12px; margin-bottom: 24px; text-align: left; font-size: 14px;">
                    <div style="margin-bottom: 8px;"><strong>Business Name:</strong> <span id="review-name"></span></div>
                    <div style="margin-bottom: 8px;"><strong>Type:</strong> <span id="review-type"></span></div>
                    <div style="margin-bottom: 8px;"><strong>First Product:</strong> <span id="review-product"></span></div>
                    <div style="margin-bottom: 8px;"><strong>Domain:</strong> <span id="review-domain"></span></div>
                </div>

                <div style="display: flex; gap: 12px;">
                    <button class="btn btn-secondary" onclick="prevStep()">Back</button>
                    <button class="btn btn-magic" onclick="publishBusiness()" id="publish-btn">
                        <span class="btn-text">Publish my business</span>
                        <div class="spinner"></div>
                    </button>
                </div>
            </div>

            <!-- Step 9: Success & Checklist -->
            <div id="step-9" class="step-container hidden">
                <h1 style="color: var(--primary); margin-bottom: 24px;">🎉 Success! Your business is live! 🎉</h1>

                <div style="background: rgba(255,255,255,0.05); padding: 16px; border-radius: 12px; margin-bottom: 32px; display: flex; align-items: center; justify-content: space-between;">
                    <div id="final-link" style="color: var(--primary); font-weight: bold; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">https://yourbusiness.ohc.app</div>
                    <button class="btn btn-secondary" style="width: auto; padding: 8px 16px;" onclick="copyLink()">Copy</button>
                </div>

                <button class="btn btn-primary" onclick="showChecklist()">View Welcome Checklist &rarr;</button>
            </div>

            <!-- Step 10: Welcome Checklist -->
            <div id="step-10" class="step-container hidden">
                <h2>Welcome Checklist</h2>
                <p class="subtitle">You're set up! Here's what to do next:</p>

                <ul class="checklist">
                    <li><span class="check-icon">✅</span> Business live</li>
                    <li><span class="check-icon">⬜</span> Add 3 more products</li>
                    <li><span class="check-icon">⬜</span> Connect Instagram</li>
                    <li><span class="check-icon">⬜</span> Share your link with a friend</li>
                </ul>

                <button class="btn btn-primary" onclick="goToDashboard()">Go to Dashboard</button>
            </div>

        </div>
    </div>

    <script>
        // State Management
        let currentStep = 0;
        const totalSteps = 8;
        let wizardState = {
            businessType: '',
            companyName: '',
            companyDescription: '',
            categories: [],
            productName: '',
            productPrice: '',
            productDescription: '',
            productImage: null,
            paymentMethod: '',
            theme: '',
            domainType: ''
        };

        // Initialize
        document.addEventListener('DOMContentLoaded', () => {
            // Check for saved state (Cross-Device Resume Simulation)
            const savedState = localStorage.getItem('ohcWizardState');
            if (savedState) {
                wizardState = JSON.parse(savedState);
                restoreUIFromState();
            }

            // Or try to fetch from backend (ignoring errors for standalone robustness)
            fetch('/api/onboarding/state')
                .then(r => r.json())
                .then(data => {
                    if (data && data.state && Object.keys(data.state).length > 0) {
                        try {
                            const parsed = JSON.parse(data.state);
                            if(parsed.currentStep) {
                                wizardState = parsed.wizardState || wizardState;
                                currentStep = parsed.currentStep;
                                restoreUIFromState();
                                showStep(currentStep);
                            }
                        } catch(e){}
                    }
                })
                .catch(() => console.log('Using local state'));

            showStep(currentStep);
        });

        // Navigation
        function showStep(step) {
            document.querySelectorAll('.step-container').forEach(el => el.classList.add('hidden'));
            const stepEl = document.getElementById(`step-${step}`);
            if (stepEl) {
                stepEl.classList.remove('hidden');

                // Update progress bar
                const progressContainer = document.getElementById('progress-container');
                if (step === 0 || step >= 9) {
                    progressContainer.style.display = 'none';
                } else {
                    progressContainer.style.display = 'block';
                    const percent = ((step) / totalSteps) * 100;
                    document.getElementById('progress-bar').style.width = `${percent}%`;
                }

                // Step specific logic
                if (step === 2 && wizardState.companyName) { checkRequired('companyName', 'btn-next-2'); }
                if (step === 4 && wizardState.productName) { checkRequired('productName', 'btn-next-4'); }
                if (step === 7) {
                    const name = wizardState.companyName || 'mybusiness';
                    const slug = name.toLowerCase().replace(/[^a-z0-9]/g, '');
                    document.getElementById('suggested-domain').innerText = `${slug}.ohc.app`;
                }
                if (step === 8) {
                    document.getElementById('review-name').innerText = wizardState.companyName || 'Not Set';
                    document.getElementById('review-type').innerText = wizardState.businessType || 'Not Set';
                    document.getElementById('review-product').innerText = wizardState.productName || 'Not Set';
                    const name = wizardState.companyName || 'mybusiness';
                    const slug = name.toLowerCase().replace(/[^a-z0-9]/g, '');
                    document.getElementById('review-domain').innerText = wizardState.domainType === 'Custom' ? 'Custom Domain' : `${slug}.ohc.app`;
                }
            }
            currentStep = step;
            saveStateToBackend();
        }

        function nextStep() {
            if (currentStep < 10) showStep(currentStep + 1);
        }

        function prevStep() {
            if (currentStep > 0) showStep(currentStep - 1);
        }

        // State Handlers
        function updateState(key, value) {
            wizardState[key] = value;
            localStorage.setItem('ohcWizardState', JSON.stringify(wizardState));
        }

        function selectOption(key, value, element) {
            updateState(key, value);

            // Visual update
            const parent = element.closest('.selection-grid');
            parent.querySelectorAll('.selection-card').forEach(c => c.classList.remove('selected'));
            element.classList.add('selected');

            // Enable next button
            const stepId = element.closest('.step-container').id;
            const stepNum = stepId.split('-')[1];
            const btn = document.getElementById(`btn-next-${stepNum}`);
            if(btn) btn.disabled = false;
        }

        function toggleArrayOption(key, value) {
            if (!wizardState[key]) wizardState[key] = [];
            const index = wizardState[key].indexOf(value);
            if (index > -1) {
                wizardState[key].splice(index, 1);
            } else {
                wizardState[key].push(value);
            }
            updateState(key, wizardState[key]);
        }

        function checkRequired(inputId, btnId) {
            const val = document.getElementById(inputId).value.trim();
            document.getElementById(btnId).disabled = val.length === 0;
        }

        function restoreUIFromState() {
            // Restore inputs
            if(wizardState.companyName) { document.getElementById('companyName').value = wizardState.companyName; checkRequired('companyName', 'btn-next-2'); }
            if(wizardState.companyDescription) document.getElementById('companyDescription').value = wizardState.companyDescription;
            if(wizardState.productName) { document.getElementById('productName').value = wizardState.productName; checkRequired('productName', 'btn-next-4'); }
            if(wizardState.productPrice) document.getElementById('productPrice').value = wizardState.productPrice;
            if(wizardState.productDescription) document.getElementById('productDescription').value = wizardState.productDescription;

            // Select cards are harder to restore simply without complex selectors, skipping for brevity in this mock,
            // but in a real React app this is reactive. We just ensure data is there.
        }

        // Actions
        async function generateAiDescription(sourceId, targetId) {
            const sourceText = document.getElementById(sourceId).value;
            if(!sourceText) return;

            const btn = event.currentTarget;
            btn.querySelector('.btn-text').style.opacity = '0';
            btn.querySelector('.spinner').classList.add('active');

            // Simulate API Call
            await new Promise(r => setTimeout(r, 1000));

            let desc = "";
            if (sourceId === 'companyName') {
                desc = `Welcome to ${sourceText}, your premium destination for high-quality goods and exceptional service. We pride ourselves on delivering the best to our local community.`;
            } else {
                desc = `The beautifully crafted ${sourceText} is designed to meet your everyday needs with elegance and durability. A must-have addition.`;
            }

            document.getElementById(targetId).value = desc;
            updateState(targetId, desc);

            btn.querySelector('.btn-text').style.opacity = '1';
            btn.querySelector('.spinner').classList.remove('active');
        }

        function handlePhotoUpload(input) {
            if (input.files && input.files[0]) {
                const reader = new FileReader();
                reader.onload = function(e) {
                    const preview = document.getElementById('photo-preview');
                    preview.src = e.target.result;
                    preview.style.display = 'block';
                    document.getElementById('upload-prompt').style.display = 'none';
                    document.getElementById('photo-upload-zone').classList.add('has-file');
                    updateState('productImage', 'uploaded_image_mock_path');
                }
                reader.readAsDataURL(input.files[0]);
            }
        }

        function updateLivePreview(theme) {
            const preview = document.getElementById('live-preview-box');
            preview.style.display = 'block';

            const title = document.getElementById('lp-title');
            const desc = document.getElementById('lp-desc');
            const btn = document.querySelector('.live-preview-btn');

            title.innerText = wizardState.companyName || 'My Awesome Store';

            if (theme === 'Modern') {
                preview.style.fontFamily = 'Inter, sans-serif';
                preview.style.background = '#ffffff';
                title.style.color = '#111';
                btn.style.borderRadius = '4px';
                btn.style.background = '#000';
            } else if (theme === 'Classic') {
                preview.style.fontFamily = 'Georgia, serif';
                preview.style.background = '#faf8f5';
                title.style.color = '#3e2723';
                btn.style.borderRadius = '0';
                btn.style.background = '#3e2723';
            } else if (theme === 'Playful') {
                preview.style.fontFamily = '"Comic Sans MS", cursive, sans-serif';
                preview.style.background = '#fff0f5';
                title.style.color = '#ff1493';
                btn.style.borderRadius = '20px';
                btn.style.background = '#ff69b4';
            }
        }

        async function publishBusiness() {
            const btn = document.getElementById('publish-btn');
            btn.querySelector('.btn-text').style.opacity = '0';
            btn.querySelector('.spinner').classList.add('active');

            // Send actual request to backend
            try {
                await fetch('/api/onboarding/start', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({
                        business_type: wizardState.businessType || 'Retail',
                        company_name: wizardState.companyName || 'My Store',
                        admin_name: 'Admin',
                        admin_email: 'admin@example.com',
                        admin_password: 'securepassword',
                        first_product_name: wizardState.productName || '',
                        first_product_price: wizardState.productPrice || '0.00',
                        price_type: 'one_time'
                    })
                });
            } catch (e) {
                console.error("Publish error, proceeding anyway for UX", e);
            }

            await new Promise(r => setTimeout(r, 1500));

            btn.querySelector('.btn-text').style.opacity = '1';
            btn.querySelector('.spinner').classList.remove('active');

            fireConfetti();

            const name = wizardState.companyName || 'mybusiness';
            const slug = name.toLowerCase().replace(/[^a-z0-9]/g, '');
            document.getElementById('final-link').innerText = `https://${slug}.ohc.app`;

            nextStep(); // Go to step 9 (Success)
        }

        function fireConfetti() {
            const canvas = document.getElementById('confetti');
            const ctx = canvas.getContext('2d');
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;

            const pieces = [];
            const colors = ['#4ecca3', '#ff69b4', '#ffd700', '#00ced1', '#ff4500'];

            for (let i = 0; i < 150; i++) {
                pieces.push({
                    x: Math.random() * canvas.width,
                    y: Math.random() * canvas.height - canvas.height,
                    w: Math.random() * 10 + 5,
                    h: Math.random() * 10 + 5,
                    color: colors[Math.floor(Math.random() * colors.length)],
                    speed: Math.random() * 3 + 2,
                    angle: Math.random() * 360,
                    spin: Math.random() * 0.2 - 0.1
                });
            }

            function animate() {
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                let active = false;
                for (let i = 0; i < pieces.length; i++) {
                    const p = pieces[i];
                    p.y += p.speed;
                    p.angle += p.spin;

                    if (p.y < canvas.height) active = true;

                    ctx.save();
                    ctx.translate(p.x, p.y);
                    ctx.rotate(p.angle);
                    ctx.fillStyle = p.color;
                    ctx.fillRect(-p.w/2, -p.h/2, p.w, p.h);
                    ctx.restore();
                }
                if (active) requestAnimationFrame(animate);
            }
            animate();
        }

        function copyLink() {
            const link = document.getElementById('final-link').innerText;
            navigator.clipboard.writeText(link);
            const btn = event.target;
            btn.innerText = "Copied!";
            setTimeout(() => btn.innerText = "Copy", 2000);
        }

        function showChecklist() {
            nextStep(); // Go to step 10
        }

        function goToDashboard() {
            window.location.href = '/';
        }

        function saveStateToBackend() {
            fetch('/api/onboarding/state', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({ currentStep, wizardState })
            }).catch(e => console.log('Backend sync failed, using localstorage only'));
        }
    </script>
</body>
</html>
"#;

pub const LOGIN_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>OneHuman - Login</title>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600&display=swap" rel="stylesheet">
    <style>
        * { box-sizing: border-box; font-family: 'Outfit', sans-serif; }
        body {
            background: linear-gradient(135deg, #1a1a2e, #16213e);
            color: white;
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }
        .login-card {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(20px);
            padding: 40px;
            border-radius: 24px;
            width: 100%;
            max-width: 400px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
        }
        h1 { margin-top: 0; text-align: center; margin-bottom: 8px; }
        p { text-align: center; color: rgba(255,255,255,0.7); margin-bottom: 32px; }
        input {
            width: 100%;
            padding: 16px;
            margin-bottom: 16px;
            border-radius: 12px;
            border: 1px solid rgba(255,255,255,0.1);
            background: rgba(255,255,255,0.05);
            color: white;
            font-size: 16px;
        }
        input:focus { outline: none; border-color: #4ecca3; }
        button {
            width: 100%;
            padding: 16px;
            background: #4ecca3;
            border: none;
            border-radius: 12px;
            color: #1a1a2e;
            font-weight: bold;
            font-size: 16px;
            cursor: pointer;
            transition: opacity 0.2s;
        }
        button:hover { opacity: 0.9; }
        .sso-btn {
            background: rgba(255,255,255,0.1);
            color: white;
            margin-top: 16px;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
        }
        .sso-btn:hover { background: rgba(255,255,255,0.15); }
        .divider {
            display: flex;
            align-items: center;
            text-align: center;
            color: rgba(255,255,255,0.5);
            margin: 24px 0;
        }
        .divider::before, .divider::after {
            content: '';
            flex: 1;
            border-bottom: 1px solid rgba(255,255,255,0.1);
        }
        .divider span { padding: 0 10px; font-size: 14px; }
    </style>
</head>
<body>
    <div class="login-card">
        <h1>Welcome Back</h1>
        <p>Log in to manage your business.</p>

        <input type="email" placeholder="Email address" id="email" />
        <input type="password" placeholder="Password" id="password" />

        <button onclick="login()">Login</button>

        <div class="divider"><span>OR</span></div>

        <button class="sso-btn" onclick="login()">Continue with Google</button>
        <button class="sso-btn" onclick="login()">Continue with Apple</button>

        <div style="text-align: center; margin-top: 24px; font-size: 14px; color: rgba(255,255,255,0.7);">
            Don't have an account? <a href="" style="color: #4ecca3; text-decoration: none;">Sign up</a>
        </div>
    </div>

    <script>
        function login() {
            // Requirement: Auto-redirect to business setup wizard on first login
            // For simulation, we always redirect to dashboard, which directs to setup
            window.location.href = '/';
        }
    </script>
</body>
</html>
"#;

pub const DASHBOARD_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>OneHuman Dashboard</title>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600&display=swap" rel="stylesheet">
    <style>
        body { font-family: 'Outfit', sans-serif; background: #0f172a; color: white; margin: 0; min-height: 100vh; display: flex; flex-direction: column; }
        nav { padding: 20px; display: flex; gap: 20px; border-bottom: 1px solid rgba(255,255,255,0.1); background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px); }
        nav a { color: white; text-decoration: none; opacity: 0.8; }
        nav a.active { color: #4ecca3; opacity: 1; }
        main { padding: 40px 20px; max-width: 1200px; margin: 0 auto; width: 100%; box-sizing: border-box; flex: 1; }
        .glass { background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(20px); border: 1px solid rgba(255,255,255,0.1); border-radius: 24px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37); }
        .card { padding: 32px; margin-bottom: 24px; text-align: center; }
        h1 { font-weight: 600; color: #4ecca3; margin-bottom: 8px; }
        .banner {
            background: linear-gradient(45deg, rgba(78, 204, 163, 0.1), rgba(168, 85, 247, 0.1));
            border: 1px solid rgba(78, 204, 163, 0.2);
            padding: 40px;
            border-radius: 24px;
            text-align: center;
            margin-bottom: 32px;
        }
        .btn {
            background: #4ecca3;
            color: #0f172a;
            padding: 16px 32px;
            border-radius: 12px;
            font-weight: 600;
            text-decoration: none;
            display: inline-block;
            margin-top: 24px;
            font-size: 18px;
            transition: transform 0.2s;
        }
        .btn:hover { transform: translateY(-2px); }

        @media (max-width: 480px) {
            main { padding: 20px 16px; }
            .banner { padding: 32px 20px; }
        }
    </style>
</head>
<body>
    <nav>
        <a href="/" class="active">Dashboard</a>
        <a href="/agents">Agents</a>
    </nav>
    <main>
        <div class="banner glass">
            <h1>Welcome to OneHuman.</h1>
            <p style="font-size: 20px; color: rgba(255,255,255,0.8); margin-bottom: 8px;">Your business, live in minutes.</p>
            <p style="color: rgba(255,255,255,0.5);">Complete the setup wizard to launch your online presence.</p>
            <a href="/business-setup" class="btn">Start Setup</a>
        </div>

        <div class="card glass" style="text-align: left;">
            <h2>Recent Activity</h2>
            <p style="color: rgba(255,255,255,0.5);">No activity yet. Set up your business to get started.</p>
        </div>
    </main>
</body>
</html>
"#;
