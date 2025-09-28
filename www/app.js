import init, {
    generate_scale,
    generate_chord,
    get_available_scales,
    get_available_modes,
    get_available_chord_qualities,
    get_available_chord_numbers,
    get_chromatic_pitches
} from './pkg/rust_music_theory.js';

let wasmInitialized = false;

// Initialize WASM module
async function initWasm() {
    try {
        await init();
        wasmInitialized = true;
        console.log('WASM module initialized successfully');

        // Enable controls after WASM is loaded
        enableControls();
    } catch (error) {
        console.error('Failed to initialize WASM module:', error);
        showError('Failed to load music theory engine. Please refresh the page.');
    }
}

function enableControls() {
    const buttons = document.querySelectorAll('.generate-btn');
    buttons.forEach(btn => {
        btn.disabled = false;
        btn.textContent = btn.textContent.replace('Loading...', btn.id.includes('scale') ? 'Generate Scale' : 'Generate Chord');
    });
}

function showError(message) {
    const errorDiv = document.createElement('div');
    errorDiv.className = 'error';
    errorDiv.textContent = message;
    document.querySelector('.container').prepend(errorDiv);

    setTimeout(() => {
        errorDiv.remove();
    }, 5000);
}

// Tab switching functionality
function showTab(tabName) {
    // Hide all tab contents
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.classList.remove('active');
    });

    // Remove active class from all tab buttons
    document.querySelectorAll('.tab-button').forEach(btn => {
        btn.classList.remove('active');
    });

    // Show selected tab content
    document.getElementById(tabName).classList.add('active');

    // Add active class to clicked button
    event.target.classList.add('active');
}

// Make showTab globally available
window.showTab = showTab;

// Generate scale functionality
function generateScale() {
    if (!wasmInitialized) {
        showError('Music theory engine not ready. Please wait...');
        return;
    }

    const tonic = document.getElementById('scale-tonic').value;
    const scaleType = document.getElementById('scale-type').value;
    const mode = document.getElementById('scale-mode').value || null;
    const octave = parseInt(document.getElementById('scale-octave').value);
    const ascending = document.getElementById('scale-direction').value === 'true';

    try {
        const result = generate_scale(tonic, scaleType, octave, mode, ascending);

        if (result && result.notes) {
            displayNotesWithPiano(result.notes, 'scale-output', 'scale-piano');
        } else {
            showError('Could not generate scale with the selected parameters.');
        }
    } catch (error) {
        console.error('Error generating scale:', error);
        showError('Error generating scale. Please try different parameters.');
    }
}

// Generate chord functionality
function generateChord() {
    if (!wasmInitialized) {
        showError('Music theory engine not ready. Please wait...');
        return;
    }

    const root = document.getElementById('chord-root').value;
    const quality = document.getElementById('chord-quality').value;
    const number = document.getElementById('chord-number').value;

    try {
        const result = generate_chord(root, quality, number);

        if (result && result.notes) {
            displayNotesWithPiano(result.notes, 'chord-output', 'chord-piano');
        } else {
            showError('Could not generate chord with the selected parameters.');
        }
    } catch (error) {
        console.error('Error generating chord:', error);
        showError('Error generating chord. Please try different parameters.');
    }
}

// Display notes in the UI
function displayNotes(notes, outputId) {
    const outputElement = document.getElementById(outputId);

    if (!notes || notes.length === 0) {
        outputElement.innerHTML = '<div class="empty-state">No notes to display</div>';
        return;
    }

    const notesHtml = notes.map(note => {
        const isBlackKey = note.pitch.includes('#') || note.pitch.includes('b');
        const className = isBlackKey ? 'note black-key' : 'note';
        return `<div class="${className}">${note.display}</div>`;
    }).join('');

    outputElement.innerHTML = notesHtml;
}

// Piano visualization
const PIANO_NOTES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
const BLACK_KEYS = ['C#', 'D#', 'F#', 'G#', 'A#'];

function createPianoVisualization(pianoId) {
    const pianoElement = document.getElementById(pianoId);
    const pianoHtml = PIANO_NOTES.map(note => {
        const isBlack = BLACK_KEYS.includes(note);
        const className = isBlack ? 'piano-key black' : 'piano-key';
        return `<div class="${className}" data-note="${note}"></div>`;
    }).join('');

    pianoElement.innerHTML = pianoHtml;
}

function highlightPianoKeys(notes, pianoId) {
    const pianoElement = document.getElementById(pianoId);
    const keys = pianoElement.querySelectorAll('.piano-key');

    // Reset all keys
    keys.forEach(key => key.classList.remove('active'));

    // Highlight active notes
    if (notes && notes.length > 0) {
        notes.forEach(note => {
            // Extract just the note name (without octave)
            const noteName = note.pitch.replace(/\d+/g, '');
            const key = pianoElement.querySelector(`[data-note="${noteName}"]`);
            if (key) {
                key.classList.add('active');
            }
        });
    }
}

function displayNotesWithPiano(notes, outputId, pianoId) {
    displayNotes(notes, outputId);
    highlightPianoKeys(notes, pianoId);
}

// Add smooth animations for note appearance
function animateNotes(outputId) {
    const notes = document.querySelectorAll(`#${outputId} .note`);
    notes.forEach((note, index) => {
        note.style.opacity = '0';
        note.style.transform = 'translateY(20px)';

        setTimeout(() => {
            note.style.transition = 'all 0.3s ease';
            note.style.opacity = '1';
            note.style.transform = 'translateY(0)';
        }, index * 100);
    });
}

// Enhanced display function with animations
function displayNotesWithAnimation(notes, outputId) {
    displayNotes(notes, outputId);
    setTimeout(() => animateNotes(outputId), 50);
}

// Update display functions to use animations
function generateScaleWithAnimation() {
    generateScale();
    const outputElement = document.getElementById('scale-output');
    setTimeout(() => {
        if (outputElement.children.length > 0 && !outputElement.querySelector('.empty-state')) {
            animateNotes('scale-output');
        }
    }, 50);
}

function generateChordWithAnimation() {
    generateChord();
    const outputElement = document.getElementById('chord-output');
    setTimeout(() => {
        if (outputElement.children.length > 0 && !outputElement.querySelector('.empty-state')) {
            animateNotes('chord-output');
        }
    }, 50);
}

// Initialize the application
document.addEventListener('DOMContentLoaded', async () => {
    // Disable buttons initially
    const buttons = document.querySelectorAll('.generate-btn');
    buttons.forEach(btn => {
        btn.disabled = true;
        btn.textContent = 'Loading...';
    });

    // Set up event listeners
    document.getElementById('generate-scale').addEventListener('click', generateScaleWithAnimation);
    document.getElementById('generate-chord').addEventListener('click', generateChordWithAnimation);

    // Create piano visualizations
    createPianoVisualization('scale-piano');
    createPianoVisualization('chord-piano');

    // Initialize WASM
    await initWasm();

    // Generate default examples
    if (wasmInitialized) {
        generateScaleWithAnimation();
        generateChordWithAnimation();
    }
});

// Keyboard shortcuts
document.addEventListener('keydown', (event) => {
    if (event.ctrlKey || event.metaKey) {
        switch (event.key) {
            case '1':
                event.preventDefault();
                showTab('scales');
                break;
            case '2':
                event.preventDefault();
                showTab('chords');
                break;
            case 'Enter':
                event.preventDefault();
                const activeTab = document.querySelector('.tab-content.active');
                if (activeTab.id === 'scales') {
                    generateScaleWithAnimation();
                } else {
                    generateChordWithAnimation();
                }
                break;
        }
    }
});

// Add hover effects for better interactivity
document.addEventListener('DOMContentLoaded', () => {
    const controls = document.querySelectorAll('select');
    controls.forEach(control => {
        control.addEventListener('change', () => {
            control.style.transform = 'scale(1.02)';
            setTimeout(() => {
                control.style.transform = 'scale(1)';
            }, 150);
        });
    });
});